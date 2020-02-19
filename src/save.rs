use serde::{Deserialize, Serialize};

use crate::data::Position;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub position: Position,
}
