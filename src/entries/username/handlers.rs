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
    let path_agent = path_from_str(&agent_info!()?.agent_initial_pubkey.to_string());
    let links_agent = get_links!(path_agent.hash()?, LinkTag::new("profile"))?;

    if links_agent.clone().into_inner().into_iter().len() <= 0 {
        // create username for this agent

        // check if the username is already taken
        // TODO: use the single_author property in entry_def. This current implementation
        // will be problematic in network partition.
        let path_usernames = path_from_str("usernames");
        let links_usernames = get_links!(path_usernames.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

        if links_usernames.clone().into_inner().into_iter().len() <= 0 {
            // username is available

            // construct UsernameEntry from input
            let username_entry = UsernameEntry {
                username: username_input.0.clone(),
                agent_id: agent_info!()?.agent_initial_pubkey
            };
        
            // commit UsernameEntry to DHT
            let username_header_address = create_entry!(&username_entry)?;
            
            // path from "usernames"
            create_link!(
                hash_entry!(path_from_str("usernames"))?,
                hash_entry!(&username_entry)?,
                LinkTag::new(username_input.0.clone().to_string())
            )?;
        
            // path from agent address
            create_link!(
                hash_entry!(path_from_str(&agent_info!()?.agent_latest_pubkey.to_string()))?, 
                hash_entry!(&username_entry)?,
                LinkTag::new("username")
            )?;
        
            // get committed profile for return value
            let username_element = get!(username_header_address.clone())?;
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
        } else {
            // username is not available
            return crate::error("This username is already taken")
        }
    } else {
        // username for this agent already exists
        return crate::error("This agent already has a username")
    }
}

pub(crate) fn get_username(agent_pubkey: AgentPubKey) -> ExternResult<UsernameOutput> {

    let path = path_from_str(&agent_pubkey.to_string());
    let links = get_links!(path.hash()?)?;

    if links.clone().into_inner().into_iter().len() >= 1 {
        let link = links.into_inner()[0].clone();
        match get!(link.target)? {
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
    let links = get_links!(path.hash()?)?;

    let mut username_vec: Vec<UsernameOutput> = Vec::default();
    for link in links.into_inner().into_iter() {
        if let Some(username_element) = get!(link.target)? {
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

    let path = path_from_str("usernames");
    let links = get_links!(path.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

    if links.clone().into_inner().into_iter().len() >= 1 {
        let link = links.into_inner()[0].clone();
        let return_val = match get!(link.target)? {
            Some(username_element) => {
                let header_details = username_element.header();
                Ok(header_details.author().to_owned())
            },
            _ => crate::error("Failed to convert element to entry")
        }?;
        
        Ok(return_val)
    } else {
        return crate::error("No user with that username exists")
    }
}

// pub(crate) fn get_my_username(_: ()) -> ExternResult<UsernameOutput> {
 
//     let path = path_from_str(&agent_info!()?.agent_initial_pubkey.to_string());
//     let links = get_links!(path.hash()?, LinkTag::new("profile"))?;

//     let link = links.into_inner()[0].clone();
//     let return_val = match get!(link.target)? {
//         Some(username_element) => {
//             let header_details = username_element.header();
//             match username_element.clone().into_inner().1.to_app_option::<UsernameEntry>()? {
//                 Some(username_entry) => {
//                     let username_output = UsernameOutput {
//                         username: username_entry.username,
//                         agent_id: header_details.author().to_owned(),
//                         created_at: header_details.timestamp(),
//                         entry_header_hash: username_element.header_address().to_owned()
//                     };
//                     Ok(username_output)
//                 },
//                 _ => crate::error("Failed to convert element to entry")
//             }
//         },
//         _ => crate::error("No username for this agent exists")
//     };

//     return_val
// }
