use hdk::prelude::*;

mod entries;
use entries::username;

use username::set_username::set_username_handler;
use username::get_usernames::get_usernames_handler;
use username::get_my_username::get_my_username_handler;
use username::get_all_usernames::get_all_usernames_handler;
use username::get_agent_pubkey_from_username::get_agent_pubkey_from_username_handler;

use username::{AgentPubKeys, UsernameEntry, UsernameList, UsernameOutput, UsernameWrapper};

// ENTRY DEF DECLARATION
entry_defs![UsernameEntry::entry_def(), Path::entry_def()];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(WasmError::Guest(String::from(reason)))
}

#[hdk_extern]
fn set_username(username_input: UsernameWrapper) -> ExternResult<UsernameOutput> {
    return set_username_handler(username_input);
}

#[hdk_extern]
fn get_usernames(agent_pubkeys: AgentPubKeys) -> ExternResult<UsernameList> {
    return get_usernames_handler(agent_pubkeys);
}

#[hdk_extern]
fn get_all_usernames(_: ()) -> ExternResult<UsernameList> {
    return get_all_usernames_handler();
}

#[hdk_extern]
fn get_agent_pubkey_from_username(username_input: UsernameWrapper) -> ExternResult<AgentPubKey> {
   return get_agent_pubkey_from_username_handler(username_input);
}

#[hdk_extern]
fn get_my_username(_: ()) -> ExternResult<UsernameOutput> {
    return get_my_username_handler();
}
