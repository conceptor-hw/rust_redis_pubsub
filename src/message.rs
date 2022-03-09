use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProverMessage {
    previous_block_hash: String,
    block_height: u32,
    block_timestamp: i64,
    difficulty_target: u64,
}

impl ProverMessage {
    pub fn new(height: u32, timestamp: i64, difficulty: u64) -> ProverMessage {
        ProverMessage {
            previous_block_hash: ProverMessage::generate_id(),
            block_height: height,
            block_timestamp: timestamp,
            difficulty_target: difficulty,
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}
