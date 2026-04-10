use globset::{Glob, GlobSet, GlobSetBuilder};
use imbl::Vector;
use serde::{Deserialize, Serialize};
use tracing::Level;

use crate::tracing::CollectedEvent;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogsState {
    pub level_filter: LevelFilter,
    pub target_filter: TargetFilter,
    #[serde(skip)]
    pub cache: FilterCache,
    #[serde(skip)]
    pub expanded_row: Option<usize>,
    #[serde(skip)]
    pub expanded_height: f32,
}

#[derive(Debug, Default)]
pub struct FilterCache {
    generation: u64,
    filter_hash: u64,
    raw_event_count: usize,
    glob_set: Option<GlobSet>,
    glob_target_count: usize,
    pub filtered_events: Vec<CollectedEvent>,
}

impl FilterCache {
    pub fn rebuild_glob_set(&mut self, targets: &[Glob]) {
        if self.glob_set.is_none() || self.glob_target_count != targets.len() {
            let mut builder = GlobSetBuilder::new();
            for target in targets {
                builder.add(target.clone());
            }
            self.glob_set = Some(builder.build().unwrap());
            self.glob_target_count = targets.len();
        }
    }

    pub fn update(
        &mut self,
        events: &Vector<CollectedEvent>,
        generation: u64,
        filter_hash: u64,
        level_filter: &LevelFilter,
    ) {
        let filters_changed = self.filter_hash != filter_hash;
        let events_changed = self.generation != generation;

        if !filters_changed && !events_changed {
            return;
        }

        let glob_set = &self.glob_set;
        let matches_glob = |target: &str| glob_set.as_ref().is_some_and(|g| g.is_match(target));
        let matches = |e: &&CollectedEvent| level_filter.get(e.level) && !matches_glob(&e.target);

        if filters_changed || events.len() < self.raw_event_count {
            self.filtered_events = events.iter().filter(matches).cloned().collect();
        } else {
            let new_events = events.iter().skip(self.raw_event_count);
            self.filtered_events
                .extend(new_events.filter(matches).cloned());
        }

        self.raw_event_count = events.len();
        self.generation = generation;
        self.filter_hash = filter_hash;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelFilter {
    pub trace: bool,
    pub debug: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Hash)]
pub struct TargetFilter {
    pub input: String,
    pub targets: Vec<Glob>,
}

impl Default for LevelFilter {
    fn default() -> Self {
        Self {
            trace: true,
            debug: true,
            info: true,
            warn: true,
            error: true,
        }
    }
}

impl LevelFilter {
    pub fn get(&self, level: Level) -> bool {
        match level {
            Level::TRACE => self.trace,
            Level::DEBUG => self.debug,
            Level::INFO => self.info,
            Level::WARN => self.warn,
            Level::ERROR => self.error,
        }
    }

    pub fn hash(&self) -> u64 {
        (self.trace as u64)
            | (self.debug as u64) << 1
            | (self.info as u64) << 2
            | (self.warn as u64) << 3
            | (self.error as u64) << 4
    }
}
