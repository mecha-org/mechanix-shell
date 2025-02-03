use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct LockButtonSettings {
    pub min_time_long_press: u64,
}
