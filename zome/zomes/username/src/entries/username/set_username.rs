use hdk::prelude::*;

use super::helpers::path_from_str;

use super::{
    UsernameWrapper,
    UsernameOutput,
    UsernameEntry,
};

pub fn set_username_handler(username_input: UsernameWrapper) -> ExternResult<UsernameOutput> {
    
    // check if this agent already has a username
    let links_agent = get_links(
        agent_info()?.agent_latest_pubkey.into(),
        Some(LinkTag::new("username")),
    )?;

    if links_agent.clone().into_inner().into_iter().len() <= 0 {
        // create username for this agent

        // check if the username is already taken
        // TODO: use the single_author property in entry_def. This current implementation
        // will be problematic in network partition.
        // let path_usernames = path_from_str("usernames");
        // let links_usernames = get_links!(path_usernames.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

        // get the entry directly from the hash instead of getting it from links
        let username_entry = UsernameEntry {
            username: username_input.0.clone(),
        };
        let username_hash = hash_entry(&username_entry)?;
        let option = GetOptions::latest();
        let maybe_username = get(username_hash, option.clone())?;

        match maybe_username {
            Some(_el) => {
                // username is not available
                return crate::error("This username is already taken");
            }
            None => {
                // username is available

                // commit UsernameEntry to DHT
                let username_header_address = create_entry(&username_entry)?;

                // link from path "usernames"
                create_link(
                    hash_entry(&path_from_str("usernames"))?,
                    hash_entry(&username_entry)?,
                    LinkTag::new(username_input.0.clone().to_string()),
                )?;

                // link from agent address
                create_link(
                    agent_info()?.agent_latest_pubkey.into(),
                    hash_entry(&username_entry)?,
                    LinkTag::new("username"),
                )?;

                // get committed username for return value
                let username_element = get(username_header_address.clone(), option.clone())?;
                match username_element {
                    Some(element) => {
                        let header_details = element.header();
                        let return_val = UsernameOutput {
                            username: username_input.0,
                            agent_id: header_details.author().to_owned(),
                            created_at: header_details.timestamp(),
                            entry_header_hash: username_header_address,
                        };
                        Ok(return_val)
                    }
                    None => crate::error("Failed to convert element to entry"),
                }
            }
        }
    } else {
        // username for this agent already exists
        return crate::error("This agent already has a username");
    }
}