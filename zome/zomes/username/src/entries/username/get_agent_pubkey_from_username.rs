use hdk::prelude::*;

use super::{
    AgentPubKey,
    UsernameEntry,
    UsernameWrapper,
};

pub(crate) fn get_agent_pubkey_from_username_handler( username_input: UsernameWrapper ) -> ExternResult<AgentPubKey> {

    // get entry by its entry hash instead of links
    let username_entry = UsernameEntry { username: username_input.0};
    let username_hash = hash_entry(&username_entry)?;
    let option = GetOptions::latest();

    match get(username_hash, option)? {
        Some(element) => {
            let header_details = element.header();
            Ok(header_details.author().to_owned())
        }
        None => crate::error("The username does not exist"),
    }
}