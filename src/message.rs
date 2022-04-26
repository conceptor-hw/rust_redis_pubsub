use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Bytes;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::string::ToString;
use strum::Display;
use strum::EnumString;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockTemplate {
    previous_block_hash: String,
    block_height: u32,
    block_timestamp: i64,
    difficulty_target: u64,
}
// #[derive(EnumString, Display, Debug, Serialize, Deserialize)]
// pub enum ProverMessage<N> {
//     Notify(BlockTemplate, u64),
//     Submit(u32, u64),
//     Unsed,
// }

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

#[derive(EnumString, Display, Debug, Serialize, Deserialize)]
pub enum ProverMessage<N> {
    Notify(BlockTemplate, u64),
    Submit(u32, u64, u64),
    Canary,
}

impl<N> ProverMessage<N> {
    #[allow(dead_code)]
    pub fn version() -> &'static u16 {
        &VERSION
    }
    pub fn id(&self) -> u8 {
        match self {
            ProverMessage::Notify(..) => 1,
            ProverMessage::Submit(..) => 2,
            ProverMessage::Canary => 4,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ProverMessage::Notify(..) => "Notify",
            ProverMessage::Submit(..) => "Submit",
            ProverMessage::Canary => "Canary",
        }
    }

    /// Returns the message data as bytes.
    #[inline]
    pub fn serialize_data_into<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Self::Notify(block_template, share_difficulty) => {
                bincode::serialize_into(&mut *writer, block_template)?;
                bincode::serialize_into(&mut *writer, share_difficulty)?;
                Ok(())
            }
            Self::Submit(block_height, nonce, proof) => {
                bincode::serialize_into(&mut *writer, block_height)?;
                bincode::serialize_into(&mut *writer, nonce)?;
                bincode::serialize_into(&mut *writer, proof)?;
                Ok(())
            }
            Self::Canary => Ok(()),
        }
    }

    /// Serializes the given message into bytes.
    #[inline]
    pub fn serialize_into<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.id().to_le_bytes()[..])?;

        self.serialize_data_into(writer)
    }

    // Deserializes the given buffer into a message.
    #[inline]
    pub fn deserialize<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        // Read the message ID.
        let id: u16 = bincode::deserialize_from(&mut *reader)?;

        // Helper function to read all the remaining bytes from a reader.
        let read_to_end = |reader: &mut R| -> Result<Bytes> {
            let mut data = vec![];
            reader.read_to_end(&mut data)?;

            Ok(data.into())
        };

        // Deserialize the data field.
        let message = match id {
            0 => Self::Notify(
                bincode::deserialize_from(&mut *reader)?,
                bincode::deserialize_from(&mut *reader)?,
            ),
            1 => Self::Submit(
                bincode::deserialize_from(&mut *reader)?,
                bincode::deserialize_from(&mut *reader)?,
                bincode::deserialize_from(&mut *reader)?,
            ),
            _ => return Err(("Invalid message ID")),
        };

        Ok(message)
    }
}
