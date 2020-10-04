use hdk3::prelude::*;
use hdk3::hash_path::path::Component;

use super::{
    UsernameEntry,
    UsernameOutput,
    UsernameList,
    // HashWrapper,
    UsernameWrapper,
    // AgentKeyWrapper
};


fn path_from_str(string_slice: &str) -> Path {
    let path = Path::from(string_slice);
    // Tats: expect() panics, should we use it?
    path.ensure().expect("Path could not be ensured");
    path
}

fn username_path(username: &str) -> Path {
    let _username_len = username.len();
    let shard: &str = "1:";
    let string_path = format!("{}{}#{}", shard, 1, username);

    let path = Path::from(string_path);
    path.ensure().expect("path could not be ensured");
    path
}

pub(crate) fn create_username(username_input: UsernameWrapper) -> ExternResult<UsernameOutput> {

    // gets all usernames to check for conflicts
    // TODO: make username entry a single_author entry to 
    // cope with partition.
    let path = path_from_str("usernames");

    let links = get_links!(path.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

    if links.clone().into_inner().into_iter().len() > 0 {
        return crate::error("An existing username exists")
    } else {
        // why are we constructing a vector here?
        let mut username_vec: Vec<UsernameOutput> = Vec::default();
        for link in links.into_inner().into_iter() {
            debug!(format!("link nicko: {:?}", link))?;
            if let Some(username_element) = get!(link.target)? {
                let header_details = username_element.header();
                debug!(format!("username_element nicko: {:?}", username_element))?;
    
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
        // construct UsernameEntry from input
        let username_entry = UsernameEntry {
            username: username_input.0.clone(),
            agent_id: agent_info!()?.agent_latest_pubkey,
        };
    
        // commit UsernameEntry to DHT
        let username_header_address = create_entry!(&username_entry)?;
        
        // path from "usernames"
        create_link!(
            hash_entry!(path_from_str("usernames"))?,
            hash_entry!(&username_entry)?,
            LinkTag::new(username_input.0.clone().to_string())
        )?;
    
        // sharded path
        // TATS: based on the explanation guillem gave us abouth path, "sharding" of path happens
        // automatically if we build paths like "usernames.a.al.ali.alic.alice", there will be a Path for
        // each string separated by dot and each of them will be linked to form a tree e.g. "usernames" -> "usernames.a" -> "usernames.a.al" so on 
        // which means we just have to commit the "youngest" (idk what word is proper) children path 
        // which in this case is "usernames.a.al.ali.alic.alice"
        create_link!(
            hash_entry!(username_path(&username_input.0))?, 
            hash_entry!(&username_entry)?
        )?;
    
        // path from agent address
        // TATS: Is it better to just link from agentpubkey directly to username entry?
        create_link!(
            hash_entry!(path_from_str(&agent_info!()?.agent_latest_pubkey.to_string()))?, 
            hash_entry!(&username_entry)?,
            LinkTag::new("username")
        )?;
    
        // get committed username for return value
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
            None => crate::error("Failed to create username")
        }
    }
}

pub fn _test_path_usernamesa(username_input: UsernameWrapper) -> ExternResult<UsernameList> {

    let mut component_vec: Vec<Component> = Vec::default();

    for letter in username_input.0.chars() {
        debug!(format!("letter is {:?}", letter.clone()))?;
        let component = Component::from(&letter.to_string());
        component_vec.push(component);
    }

    debug!(format!("component vector: {:?}", component_vec))?;

    let path = Path::from(component_vec);
    let links = get_links!(path.hash()?)?;

    let mut username_vec: Vec<UsernameOutput> = Vec::default();

    for link in links.into_inner().into_iter() {
        if let Some(username_element) = get!(link.target)? {
            let header_details = username_element.header();
            debug!(format!("Element found. Converting {:?}...", username_element))?;

            if let Some(username_entry) = username_element.to_owned().into_inner().1.to_app_option::<UsernameEntry>()? {
                
                debug!(format!("Entry found"))?;

                let username_output = UsernameOutput {
                    username: username_entry.username,
                    agent_id: header_details.author().to_owned(),
                    created_at: header_details.timestamp(),
                    entry_header_hash: username_element.header_address().to_owned()
                };

                debug!(format!("Successfully converted. Pushing now..."))?;
                username_vec.push(username_output)
            } else {
                debug!(format!("Cannot convert from entry"))?;
                continue                
            }
        } else {
            debug!(format!("No element found at link"))?;
            continue
        }
    };

    Ok(username_vec.into())


    // let path = username_path(&profile_input.username);
    // a returns alice
    // al does not return alice
    // let path = path_from_str("a");
    // debug!(format!("root link: {:?}", path))?;

    // let children_links = path.children()?.into_inner();
    // debug!(format!("children links: {:?}", children_links))?;
    // debug!(format!("number of children: {:?}", children_links.len()))?;

    // let mut profile_vec: Vec<ProfileOutput> = Vec::new(); 

    // for shard in children_links {
    //     debug!(format!("shard step: {:?}", shard.clone()))?;

    //     let granchildren_links = get_links!(hash_entry!(shard.clone())?)?;

    //     debug!(format!("grandchildren: {:?}", granchildren_links))?;

    //     match get!(hash_entry!(shard)?)? {
    //         Some(element) => {
    //             let header_details = element.header();
    //             match element.clone().into_inner().1.to_app_option::<ProfileEntry>()? {
    //                 Some(entry) => {
    //                     let profile_output = ProfileOutput {
    //                         username: entry.username,
    //                         agent_id: header_details.author().to_owned(),
    //                         created_at: header_details.timestamp(),
    //                         entry_header_hash: element.header_address().to_owned()
    //                     };
    //                     profile_vec.push(profile_output)
    //                 },
    //                 _ => continue_vec: Vec<ProfileOutput> = Vec::new(); 

    // for shard in children_links {
    //     debug!(format!("shard step: {:?}", shard.clone()))?;

    //     let granchildren_links = get_links!(hash_entry!(shard.clone())?)?;

    //     debug!(format!("grandchildren: {:?}", granchildren_links))?;

    //     match get!(hash_entry!(shard)?)? {
    //         Some(element) => {
    //             let header_details = element.header();
    //             match element.clone().into_inner().1.to_app_option::<ProfileEntry>()? {
    //                 Some(entry) => {
    //                     let profile_output = ProfileOutput {
    //                         username: entry.username,
    //                         agent_id: header_details.author().to_owned(),
    //                         created_at: header_details.timestamp(),
    //                         entry_header_hash: element.header_address().to_owned()
    //                     };
    //                     profile_vec.push(profile_output)
    //                 },
    //                 _ => continue
    //             }let mut profile_vec: Vec<ProfileOutput> = Vec::new();

    //         },
    //         _ => {
    //             debug!(format!("no element found at"))?;
    //             continue
    //         }
    //     }   
    // };

    // Ok(profile_vec
    //             }
    //         },
    //         _ => {
    //             debug!(format!("no element found at"))?;
    //             continue
    //         }
    //     }   
    // };

    // Ok(profile_vec.into())
}

// Tats: What is this function gonna be use for?
// Just wondering since there was no similar function in old kizuna.
pub(crate) fn get_profile_from_username (username_input: UsernameWrapper) -> ExternResult<UsernameList> {
    let path = path_from_str("usernames");

    let links = get_links!(path.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

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

pub(crate) fn get_my_username(_: ()) -> ExternResult<UsernameOutput> {
    let path = path_from_str(&agent_info!()?.agent_initial_pubkey.to_string());

    let links = get_links!(path.hash()?, LinkTag::new("profile"))?;

    let link = links.into_inner()[0].clone();
    
    let return_val = match get!(link.target)? {
        Some(username_element) => {
            let header_details = username_element.header();
            match username_element.clone().into_inner().1.to_app_option::<UsernameEntry>()? {
                Some(username_entry) => {
                    let username_output = UsernameOutput {
                        username: username_entry.username,
                        agent_id: header_details.author().to_owned(),
                        created_at: header_details.timestamp(),
                        entry_header_hash: username_element.header_address().to_owned()
                    };
                    Ok(username_output)
                },
                _ => crate::error("Failed to get username")
            }
        },
        _ => crate::error("Failed to get username")
    };

    return_val
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

    let path = username_path(&username_input.0);
    let links = get_links!(path.hash()?)?;
    let link = links.into_inner()[0].clone();

    let return_val = match get!(link.target)? {
        Some(profile_element) => {
            let header_details = profile_element.header();
            Ok(header_details.author().to_owned())
        },
        _ => crate::error("Failed to get entry from element")
    }?;

    // let wrapped = AgentKeyWrapper(return_val?.into());
    Ok(return_val)
}