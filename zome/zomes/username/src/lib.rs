use hdk3::prelude::*;

mod entries;
use entries::username;

use username::{AgentPubKeys, UsernameEntry, UsernameList, UsernameOutput, UsernameWrapper};

// ENTRY DEF DECLARATION
entry_defs![UsernameEntry::entry_def(), Path::entry_def()];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

#[hdk_extern]
fn set_username(username_input: UsernameWrapper) -> ExternResult<UsernameOutput> {
    username::handlers::set_username(username_input)
}

#[hdk_extern]
fn get_usernames(agent_pubkeys: AgentPubKeys) -> ExternResult<UsernameList> {
    username::handlers::get_username(agent_pubkeys)
}

#[hdk_extern]
fn get_all_usernames(_: ()) -> ExternResult<UsernameList> {
    username::handlers::get_all_usernames(())
}

#[hdk_extern]
fn get_agent_pubkey_from_username(username_input: UsernameWrapper) -> ExternResult<AgentPubKey> {
    username::handlers::get_agent_pubkey_from_username(username_input)
}

#[hdk_extern]
fn get_my_username(_: ()) -> ExternResult<UsernameOutput> {
    username::handlers::get_my_username(())
}
