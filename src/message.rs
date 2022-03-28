use serde::{Deserialize, Serialize};
use std::string::ToString;
use strum::Display;
use strum::EnumString;
use uuid::Uuid;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Display;
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockTemplate {
    previous_block_hash: String,
    block_height: u32,
    block_timestamp: i64,
    difficulty_target: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ProverMessage {
    Notify(BlockTemplate, u64),
}

impl Display for ProverMessage {
    fn fmt(&self, f: &mut Formatter) ->  fmt::Result  {
        match self {
            ProverMessage::Notify(tmp,i) => write!(f, "{:?} {}", tmp,i),
		}
    }
}

impl BlockTemplate {
    pub fn new(height: u32, timestamp: i64, difficulty: u64) -> BlockTemplate {
        BlockTemplate {
            previous_block_hash: BlockTemplate::generate_id(),
            block_height: height,
            block_timestamp: timestamp,
            difficulty_target: difficulty,
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}

// impl ProverMessage {
//     pub fn new(height: u32, timestamp: i64, difficulty: u64) -> ProverMessage {
//         BlockTemplate {
//             previous_block_hash: ProverMessage::generate_id(),
//             block_height: height,
//             block_timestamp: timestamp,
//             difficulty_target: difficulty,
//         }
//     }

//     fn generate_id() -> String {
//         Uuid::new_v4().to_simple().to_string()
//     }
// }

// impl fmt::Display for ProverMessage {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.previous_block_hash, self.block_height,self.block_timestamp, self.difficulty_target)
//     }
// }

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

pub const SUB_PROVER_SPEC_MESSAGE: &str = "prover_spec_msg_channel_for_pool";
pub const PUB_PROVER_SPEC_MESSAGE: &str = "prover_spec_msg_channel_pool";

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveSpecMessage {
    pub Prover_id: String,
    pub Info: String,
}

impl ProveSpecMessage {
    pub fn new(id: String, msg: String) -> ProveSpecMessage {
        ProveSpecMessage {
            Prover_id: id,
            Info: msg,
        }
    }
}
