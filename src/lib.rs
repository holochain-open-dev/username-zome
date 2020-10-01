use hdk3::prelude::*;
use hdk3::prelude::Path;
mod entries;
use entries::profile;

use profile::{
    ProfileEntry,
    ProfileInput,
    ProfileOutput,
    ProfileList,
    UsernameWrapper,
    AgentKeyWrapper
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
    ProfileEntry::entry_def()
];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

#[hdk_extern]
fn create_profile(profile_input: ProfileInput) -> ExternResult<ProfileOutput> {
    profile::handlers::create_profile(profile_input)
}

#[hdk_extern]
fn get_profile_from_username (username_input: UsernameWrapper) -> ExternResult<ProfileList> {
    profile::handlers::get_profile_from_username(username_input)
}

#[hdk_extern]
fn get_my_profile(_: ()) -> ExternResult<ProfileOutput> {
    profile::handlers::get_my_profile(())
}

#[hdk_extern]
fn get_all_profiles(_: ()) -> ExternResult<ProfileList> {
    profile::handlers::get_all_profiles(())
}

#[hdk_extern]
fn get_agent_pubkey_from_username(username_input: UsernameWrapper) -> ExternResult<AgentKeyWrapper> {
    profile::handlers::get_agent_pubkey_from_username(username_input)
}