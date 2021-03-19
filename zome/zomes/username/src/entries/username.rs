use crate::timestamp::Timestamp;
use derive_more::{From, Into};
use hdk::prelude::*;

pub mod helpers;
pub mod set_username;
pub mod get_usernames;
pub mod get_all_usernames;
pub mod get_my_username;
pub mod get_agent_pubkey_from_username;

#[hdk_entry(id = "username", visibility = "public")]
pub struct UsernameEntry {
    username: String,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, From, Into, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsernameOutput {
    username: String,
    agent_id: AgentPubKey,
    created_at: Timestamp,
    entry_header_hash: HeaderHash,
}

#[derive(From, Into, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct UsernameList(Vec<UsernameOutput>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct UsernameWrapper(String);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct AgentPubKeys(Vec<AgentPubKey>);
