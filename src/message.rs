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

pub const SUB_BINARY_CHANNEL: &str = "binary_channel_schedule";
pub const PUB_BINARY_CHANNEL: &str = "binary_channel_prover";
pub const SUB_MGT_CHANNEL: &str = "mgt_channel_schedule";
pub const PUB_MGT_CHANNEL: &str = "mgt_channel_prover";

//订阅发布redis message
#[derive(Debug, Serialize, Deserialize)]
pub struct PubSubMessage {
    pub id: String,
    pub channel: String,
    pub payload: Order,
}

impl PubSubMessage {
    pub fn new(payload: Order) -> PubSubMessage {
        PubSubMessage {
            id: PubSubMessage::generate_id(),
            channel: PUB_MGT_CHANNEL.to_string(),
            payload,
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub description: String,
    pub quantity: u64,
    pub index: i32,
}

impl Order {
    pub fn new(description: String, quantity: u64, index: i32) -> Order {
        Order {
            description,
            quantity,
            index,
        }
    }
}
