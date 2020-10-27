use hdk3::prelude::*;
use derive_more::{From, Into};
use crate::{timestamp::Timestamp};
pub mod handlers;

#[hdk_entry(id = "username", visibility = "public" )]
pub struct UsernameEntry {
    username: String,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, From, Into, Debug)]
pub struct UsernameOutput {
    username: String,
    agent_id: AgentPubKey,
    created_at: Timestamp,
    entry_header_hash: HeaderHash
}

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct UsernameList(Vec<UsernameOutput>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct UsernameWrapper(String);