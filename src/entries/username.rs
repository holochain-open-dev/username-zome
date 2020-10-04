use hdk3::prelude::*;
use derive_more::{From, Into};
use crate::{timestamp::Timestamp};
pub mod handlers;

#[hdk_entry(id = "usernaem", visibility = "public" )]
pub struct UsernameEntry {
    username: String,
    agent_id: AgentPubKey,
}

// Tats: We could take this out since we already have a UsernameWrapper?
// #[derive(Serialize, Deserialize, SerializedBytes)]
// pub struct UsernameInput {
//     username: String
// }

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
pub struct HashWrapper(AnyDhtHash);

// Tats: We probably dont need this since we can just do let x = HashWrapper(foo);?
impl HashWrapper {
    pub fn new(hash: AnyDhtHash) -> Self {
        HashWrapper(hash)
    }
}

// Tats: Maybe we don't need this since AgentPubKey already has these attributes?
// #[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
// pub struct AgentKeyWrapper(AgentPubKey);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct UsernameWrapper(String);

// Tats: We probably don't need this since we can just do let x = UsernameWrapper("tats")?
// impl UsernameWrapper {
//     pub fn new(username: String) -> Self {
//         UsernameWrapper(username)
//     }
// }