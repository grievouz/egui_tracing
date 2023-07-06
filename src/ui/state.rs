use std::collections::HashSet;

use globset::Glob;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tracing::Level;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogsState {
    pub level_filter: LevelFilter,
    pub target_filter: TargetFilter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelFilter {
    pub trace: bool,
    pub debug: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TargetFilter {
    pub input: String,
    #[serde_as(as = "HashSet<_>")]
    pub targets: HashSet<Glob>,
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
}
