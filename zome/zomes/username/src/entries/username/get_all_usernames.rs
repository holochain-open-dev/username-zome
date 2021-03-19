use hdk::prelude::*;

use super::helpers::path_from_str;

use super::{
    UsernameList,
    UsernameOutput,
    UsernameEntry,
};

pub fn get_all_usernames_handler() -> ExternResult<UsernameList> {
    let path = path_from_str("usernames");
    let links = get_links(path.hash()?, None)?;

    let mut username_vec: Vec<UsernameOutput> = Vec::default();
    for link in links.into_inner().into_iter() {
        let option = GetOptions::latest();
        if let Some(username_element) = get(link.target, option)? {
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
                username_vec.push(username_output)
            }
        } else {
            continue;
        }
    }

    Ok(username_vec.into())
}