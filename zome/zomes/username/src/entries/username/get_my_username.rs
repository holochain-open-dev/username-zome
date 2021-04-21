use hdk::prelude::*;

use super::{
    UsernameOutput,
    UsernameEntry,
};

pub fn get_my_username_handler() -> ExternResult<UsernameOutput> {
    let query_result:Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Public,
            )))
            .include_entries(true),
    )?;

    let map_result: Vec<UsernameOutput> = query_result
        .into_iter()
        .filter_map(|el| {
            let header_details = el.header();
            let entry = el.clone().into_inner().1.to_app_option::<UsernameEntry>();
            match entry {
                Ok(Some(username_entry)) => {
                    let username_output = UsernameOutput {
                        username: username_entry.username,
                        agent_id: header_details.author().to_owned(),
                        created_at: header_details.timestamp(),
                        entry_header_hash: el.header_address().to_owned(),
                    };
                    Some(username_output)
                }
                _ => None,
            }
        })
        .collect();

    if map_result.len() == 1 {
        return Ok(map_result[0].clone());
    } else {
        return crate::error("No username exists for this agent");
    }
}
