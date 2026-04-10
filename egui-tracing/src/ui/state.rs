use std::hash::{Hash, Hasher};

use globset::{Glob, GlobSet, GlobSetBuilder};
use imbl::Vector;
use tracing::Level;

use crate::tracing::CollectedEvent;

#[derive(Debug, Default)]
pub struct LogsState {
    pub level_filter: LevelFilter,
    pub target_filter: TargetFilter,
    pub cache: FilterCache,
    pub selected_row: Option<usize>,
    pub scroll_to_bottom: bool,
}

#[derive(Debug, Default)]
pub struct FilterCache {
    generation: u64,
    filter_hash: u64,
    raw_event_count: usize,
    glob_set: Option<GlobSet>,
    glob_targets_hash: u64,
    source_events: Vector<CollectedEvent>,
    filtered_indices: Vec<usize>,
}

impl FilterCache {
    pub fn needs_update(&self, generation: u64, filter_hash: u64) -> bool {
        self.generation != generation || self.filter_hash != filter_hash
    }

    pub fn len(&self) -> usize {
        self.filtered_indices.len()
    }

    pub fn get(&self, idx: usize) -> &CollectedEvent {
        &self.source_events[self.filtered_indices[idx]]
    }

    pub fn rebuild_glob_set(&mut self, targets: &[Glob]) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        targets.hash(&mut hasher);
        let new_hash = hasher.finish();

        if self.glob_set.is_none() || self.glob_targets_hash != new_hash {
            let mut builder = GlobSetBuilder::new();
            for target in targets {
                builder.add(target.clone());
            }
            self.glob_set = Some(builder.build().unwrap());
            self.glob_targets_hash = new_hash;
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
        let matches =
            |e: &CollectedEvent| level_filter.get(e.level) && !matches_glob(&e.target);

        if filters_changed || events.len() < self.raw_event_count {
            self.filtered_indices = events
                .iter()
                .enumerate()
                .filter(|(_, e)| matches(e))
                .map(|(i, _)| i)
                .collect();
        } else {
            let start = self.raw_event_count;
            let new_events = events.skip(start);
            let new_indices = new_events
                .iter()
                .enumerate()
                .filter(|(_, e)| matches(e))
                .map(|(i, _)| i + start);
            self.filtered_indices.extend(new_indices);
        }

        self.source_events = events.clone();
        self.raw_event_count = events.len();
        self.generation = generation;
        self.filter_hash = filter_hash;
    }
}

#[derive(Debug)]
pub struct LevelFilter {
    pub trace: bool,
    pub debug: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

#[derive(Debug, Default, Clone, Hash)]
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
