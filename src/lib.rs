use hdk3::prelude::*;
use hdk3::prelude::Path;
mod entries;
use entries::username;

use username::{
    UsernameEntry,
    UsernameOutput,
    UsernameList,
    UsernameWrapper,
};

// pub use entries::{
//     profile::{
//         ProfileEntry,
//         ProfileInput,
//         ProfileOutput,
//     },
//     profile::*
// };

// ENTRY DEF DECLARATION
entry_defs![
    Path::entry_def(),
    UsernameEntry::entry_def()
];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

#[hdk_extern]
fn create_username(username_input: UsernameWrapper) -> ExternResult<UsernameOutput> {
    username::handlers::create_username(username_input)
}

#[hdk_extern]
fn get_profile_from_username (username_input: UsernameWrapper) -> ExternResult<UsernameList> {
    username::handlers::get_profile_from_username(username_input)
}

#[hdk_extern]
fn get_my_username(_: ()) -> ExternResult<UsernameOutput> {
    username::handlers::get_my_username(())
}

#[hdk_extern]
fn get_all_usernames(_: ()) -> ExternResult<UsernameList> {
    username::handlers::get_all_usernames(())
}

#[hdk_extern]
fn get_agent_pubkey_from_username(username_input: UsernameWrapper) -> ExternResult<AgentPubKey> {
    username::handlers::get_agent_pubkey_from_username(username_input)
}