use hdk3::prelude::*;
use link::Link;
use hdk3::hash_path::path::Component;

use crate::entries::profile::{
    ProfileEntry,
    ProfileInput,
    ProfileOutput,
    ProfileList,
    HashWrapper,
    UsernameWrapper
};


fn path_from_str(string_slice: &str) -> Path {
    let path = Path::from(string_slice);
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

#[hdk_extern]
pub fn create_profile(profile_input: ProfileInput) -> ExternResult<ProfileOutput> {

    // gets all profiles to check for conflicts
    let path = path_from_str("profiles");

    let links = get_links!(path.hash()?, LinkTag::new(profile_input.username.clone().to_string()))?;

    if links.clone().into_inner().into_iter().len() > 0 {
        return crate::error("An existing profile with the same username exists")
    } else {
        let mut profile_vec: Vec<ProfileOutput> = Vec::new();
        for link in links.into_inner().into_iter() {
            debug!(format!("link nicko: {:?}", link))?;
            if let Some(profile_element) = get!(link.target)? {
                let header_details = profile_element.header();
                debug!(format!("profile_element nicko: {:?}", profile_element))?;
    
                if let Some(profile_entry) = profile_element.clone().into_inner().1.to_app_option::<ProfileEntry>()? {
                    let profile_output = ProfileOutput {
                        username: profile_entry.username,
                        agent_id: header_details.author().to_owned(),
                        created_at: header_details.timestamp(),
                        entry_header_hash: profile_element.header_address().to_owned()
                    };
                    profile_vec.push(profile_output)
                }
            } else {
                continue
            }
        };
    
        // construct ProfileEntry from input
        let profile_entry = ProfileEntry {
            username: profile_input.username.clone(),
            agent_id: agent_info!()?.agent_initial_pubkey
        };
    
        // commit ProfileEntry to DHT
        let profile_header_address = create_entry!(&profile_entry)?;
        
        // path from "profiles"
        create_link!(
            hash_entry!(path_from_str("profiles"))?,
            hash_entry!(&profile_entry)?,
            LinkTag::new(profile_input.username.clone().to_string())
        )?;
    
        // sharded path
        create_link!(
            hash_entry!(username_path(&profile_input.username))?, 
            hash_entry!(&profile_entry)?
        )?;
    
        // path from agent address
        create_link!(
            hash_entry!(path_from_str(&agent_info!()?.agent_initial_pubkey.to_string()))?, 
            hash_entry!(&profile_entry)?,
            LinkTag::new("profile")
        )?;
    
        // get committed profile for return value
        let profile_element = get!(profile_header_address.clone())?;
        match profile_element {
            Some(element) => {
                let header_details = element.header();
                let return_val = ProfileOutput {
                    username: profile_input.username,
                    agent_id: header_details.author().to_owned(),
                    created_at: header_details.timestamp(),
                    entry_header_hash: profile_header_address
                };
                Ok(return_val)
            },
            None => crate::error("Failed to create profile")
        }
    }
}

#[hdk_extern]
pub fn test_path_profile(profile_input: ProfileInput) -> ExternResult<ProfileList> {

    let mut component_vec: Vec<Component> = Vec::new();

    for letter in profile_input.username.chars() {
        debug!(format!("letter is {:?}", letter.clone()))?;
        let component = Component::from(&letter.to_string());
        component_vec.push(component);
    }

    debug!(format!("component vector: {:?}", component_vec))?;

    let path = Path::from(component_vec);
    let links = get_links!(path.hash()?)?;

    let mut profile_vec: Vec<ProfileOutput> = Vec::new();

    for link in links.into_inner().into_iter() {
        if let Some(profile_element) = get!(link.target)? {
            let header_details = profile_element.header();
            debug!(format!("Element found. Converting {:?}...", profile_element))?;

            if let Some(profile_entry) = profile_element.to_owned().into_inner().1.to_app_option::<ProfileEntry>()? {
                
                debug!(format!("Entry found"))?;

                let profile_output = ProfileOutput {
                    username: profile_entry.username,
                    agent_id: header_details.author().to_owned(),
                    created_at: header_details.timestamp(),
                    entry_header_hash: profile_element.header_address().to_owned()
                };

                debug!(format!("Successfully converted. Pushing now..."))?;
                profile_vec.push(profile_output)
            } else {
                debug!(format!("Cannot convert from entry"))?;
                continue                
            }
        } else {
            debug!(format!("No element found at link"))?;
            continue
        }
    };

    Ok(profile_vec.into())


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

#[hdk_extern]
pub fn get_profile_from_username (username_input: UsernameWrapper) -> ExternResult<ProfileList> {
    let path = path_from_str("profiles");

    let links = get_links!(path.hash()?, LinkTag::new(username_input.0.clone().to_string()))?;

    let mut profile_vec: Vec<ProfileOutput> = Vec::new();
    for link in links.into_inner().into_iter() {
        if let Some(profile_element) = get!(link.target)? {
            let header_details = profile_element.header();
            if let Some(profile_entry) = profile_element.clone().into_inner().1.to_app_option::<ProfileEntry>()? {
                let profile_output = ProfileOutput {
                    username: profile_entry.username,
                    agent_id: header_details.author().to_owned(),
                    created_at: header_details.timestamp(),
                    entry_header_hash: profile_element.header_address().to_owned()
                };
                profile_vec.push(profile_output)
            }
        } else {
            continue
        }
    };
    
    Ok(profile_vec.into())

}

#[hdk_extern]
pub fn get_my_profile(_: ()) -> ExternResult<ProfileOutput> {
    let path = path_from_str(&agent_info!()?.agent_initial_pubkey.to_string());

    let links = get_links!(path.hash()?, LinkTag::new("profile"))?;

    let link = links.into_inner()[0].clone();
    
    let return_val = match get!(link.target)? {
        Some(profile_element) => {
            let header_details = profile_element.header();
            match profile_element.clone().into_inner().1.to_app_option::<ProfileEntry>()? {
                Some(profile_entry) => {
                    let profile_output = ProfileOutput {
                        username: profile_entry.username,
                        agent_id: header_details.author().to_owned(),
                        created_at: header_details.timestamp(),
                        entry_header_hash: profile_element.header_address().to_owned()
                    };
                    Ok(profile_output)
                },
                _ => crate::error("Failed to get profile")
            }
        },
        _ => crate::error("Failed to get profile")
    };

    return_val
}

#[hdk_extern]
pub fn get_all_profiles(_: ()) -> ExternResult<ProfileList> {

    let path = path_from_str("profiles");
    let links = get_links!(path.hash()?)?;

    let mut profile_vec: Vec<ProfileOutput> = Vec::new();
    for link in links.into_inner().into_iter() {
        if let Some(profile_element) = get!(link.target)? {
            let header_details = profile_element.header();
            if let Some(profile_entry) = profile_element.clone().into_inner().1.to_app_option::<ProfileEntry>()? {
                let profile_output = ProfileOutput {
                    username: profile_entry.username,
                    agent_id: header_details.author().to_owned(),
                    created_at: header_details.timestamp(),
                    entry_header_hash: profile_element.header_address().to_owned()
                };
                profile_vec.push(profile_output)
            }
        } else {
            continue
        }
    };
    
    Ok(profile_vec.into())
}

#[hdk_extern]
pub fn get_address_from_username(username_input: UsernameWrapper) -> ExternResult<HashWrapper> {

    let path = username_path(&username_input.0);
    let links = get_links!(path.hash()?)?;
    let link = links.into_inner()[0].clone();

    let return_val = match get!(link.target)? {
        Some(profile_element) => {
            let header_details = profile_element.header();
            Ok(header_details.author().to_owned())
        },
        _ => crate::error("Failed to get entry from element")
    };

    let wrapped = HashWrapper(return_val?.into());
    Ok(wrapped)
}