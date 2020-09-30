use hdk3::prelude::*;
use derive_more::{Constructor, From, Into};
use crate::{timestamp::Timestamp};
pub mod handlers;

#[hdk_entry(id = "profile", visibility = "public" )]
pub struct ProfileEntry {
    username: String,
    agent_id: AgentPubKey,
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct ProfileInput {
    username: String
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, From, Into, Debug)]
pub struct ProfileOutput {
    username: String,
    agent_id: AgentPubKey,
    created_at: Timestamp,
    entry_header_hash: HeaderHash
}

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct ProfileList(Vec<ProfileOutput>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct HashWrapper(AnyDhtHash);

impl HashWrapper {
    pub fn new(hash: AnyDhtHash) -> Self {
        HashWrapper(hash)
    }
}

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct UsernameWrapper(String);

impl UsernameWrapper {
    pub fn new(username: String) -> Self {
        UsernameWrapper(username)
    }
}