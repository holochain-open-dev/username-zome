use hdk::prelude::*;

use super::{

    AgentPubKeys,
    UsernameList,
    UsernameEntry,
    UsernameOutput,
};

pub fn get_usernames_handler(agent_pubkeys: AgentPubKeys) -> ExternResult<UsernameList> {
    let mut username_list = Vec::default();

    for key in agent_pubkeys.0 {
        let links = get_links(key.into(), Some(LinkTag::new("username")))?;

        if links.clone().into_inner().into_iter().len() >= 1 {
            let link = links.into_inner()[0].clone();
            let option = GetOptions::content();
            match get(link.target, option)? {
                Some(username_element) => {
                    let header_details = username_element.header();
                    if let Some(username_entry) = username_element
                        .clone()
                        .into_inner()
                        .1
                        .to_app_option::<UsernameEntry>()?
                    {
                        let username_output = UsernameOutput {
                            username: username_entry.username,
                            agent_id: header_details.author().to_owned(),
                            created_at: header_details.timestamp(),
                            entry_header_hash: username_element.header_address().to_owned(),
                        };
                        username_list.push(username_output)
                    } else {
                        return crate::error("Failed to convert element to entry");
                    }
                }
                _ => return crate::error("Failed to get the username for this agent"),
            }
        } else {
            return crate::error("No username for this agent exists");
        }
    }

    Ok(UsernameList(username_list))
}