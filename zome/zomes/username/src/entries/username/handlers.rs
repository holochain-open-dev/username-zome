use hdk3::prelude::*;

use super::{
    UsernameEntry,
    UsernameOutput,
    UsernameList,
    UsernameWrapper
};

fn path_from_str(string_slice: &str) -> Path {
    let path = Path::from(string_slice);
    // Tats: expect() panics, should we use it?
    path.ensure().expect("Path could not be ensured");
    path
}

pub(crate) fn set_username(username_input: UsernameWrapper) -> ExternResult<UsernameOutput> {
    // check if this agent already has a username
    let links_agent = get_links(agent_info()?.agent_latest_pubkey.into(), LinkTag::new("username"))?;

    if links_agent.clone().into_inner().into_iter().len() <= 0 {
        // create username for this agent

        // check if the username is already taken
        // TODO: use the single_author property in entry_def. This current implementation
        // will be problematic in network partition.
        // let path_usernames = path_from_str("usernames");
        // let links_usernames = get_links!(path_usernames.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

        // get the entry directly from the hash instead of getting it from links
        let username_entry = UsernameEntry {
            username: username_input.0.clone()
        };
        let username_hash = hash_entry(&username_entry)?;
        let option = GetOptions::latest();
        let maybe_username = get(username_hash, option.clone())?;

        match maybe_username {
            Some(_el) => {
                // username is not available
                return crate::error("This username is already taken")
            },
            None => {
                // username is available
            
                // commit UsernameEntry to DHT
                let username_header_address = create_entry(&username_entry)?;
                
                // link from path "usernames"
                create_link(
                    hash_entry(path_from_str("usernames"))?,
                    hash_entry(&username_entry)?,
                    LinkTag::new(username_input.0.clone().to_string())
                )?;
            
                // link from agent address
                create_link(
                    agent_info()?.agent_latest_pubkey.into(),
                    hash_entry(&username_entry)?,
                    LinkTag::new("username")
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
                            entry_header_hash: username_header_address
                        };
                        Ok(return_val)
                    },
                    None => crate::error("Failed to convert element to entry")
                }
            }
        } 
    } else {
        // username for this agent already exists
        return crate::error("This agent already has a username")
    }
}

pub(crate) fn get_username(agent_pubkey: AgentPubKey) -> ExternResult<UsernameOutput> {

    let links = get_links(agent_pubkey.into())?;

    if links.clone().into_inner().into_iter().len() >= 1 {
        let link = links.into_inner()[0].clone();
        let option = GetOptions::latest();
        match get(link.target, option)? {
            Some(username_element) => {
                let header_details = username_element.header();
                if let Some(username_entry) = username_element.clone().into_inner().1.to_app_option::<UsernameEntry>()? {
                    let username_output = UsernameOutput {
                        username: username_entry.username,
                        agent_id: header_details.author().to_owned(),
                        created_at: header_details.timestamp(),
                        entry_header_hash: username_element.header_address().to_owned()
                    };
                    Ok(username_output)
                } else {
                    return crate::error("Failed to convert element to entry")
                }
            },
            _ => return crate::error("No username for this agent exists")
        }
    } else {
        return crate::error("No username for this agent exists")
    }

}

pub(crate) fn get_all_usernames(_: ()) -> ExternResult<UsernameList> {

    let path = path_from_str("usernames");
    let links = get_links(path.hash()?)?;

    let mut username_vec: Vec<UsernameOutput> = Vec::default();
    for link in links.into_inner().into_iter() {
        if let Some(username_element) = get(link.target)? {
            let header_details = username_element.header();
            if let Some(username_entry) = username_element.clone().into_inner().1.to_app_option::<UsernameEntry>()? {
                let username_output = UsernameOutput {
                    username: username_entry.username,
                    agent_id: header_details.author().to_owned(),
                    created_at: header_details.timestamp(),
                    entry_header_hash: username_element.header_address().to_owned()
                };
                username_vec.push(username_output)
            }
        } else {
            continue
        }
    };
    
    Ok(username_vec.into())
}

pub(crate) fn get_agent_pubkey_from_username(username_input: UsernameWrapper) -> ExternResult<AgentPubKey> {

    // let path = path_from_str("usernames");
    // let links = get_links!(path.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

    // get entry by its entry hash instead of links
    let username_entry = UsernameEntry { username: username_input.0 };
    let username_hash = hash_entry(&username_entry)?;
    let option = GetOptions::latest();
    match get(username_hash, option)? {
        Some(el) => {
            let header_details = el.header();
            Ok(header_details.author().to_owned())
        },
        None => crate::error("The username does not exist")
    } 
}

pub(crate) fn get_my_username(_: ()) -> ExternResult<UsernameOutput> {

    let query_result = query(
        QueryFilter::new()
        .entry_type(
            EntryType::App(
                AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info()?.zome_id,
                    EntryVisibility::Public
                )
            )
        )
        .include_entries(true)
    )?;

    let map_result: Vec<UsernameOutput>= query_result.0
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
                        entry_header_hash: el.header_address().to_owned()
                    };
                    Some(username_output)
                }, 
                _ => None
            }
        })
        .collect();
        
    if map_result.len() == 1 { return Ok(map_result[0].clone()) } 
    else { return crate::error("No username exists for this agent") }
}
