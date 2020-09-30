#![allow(unused_imports)]
#![allow(dead_code)]

use hdk3::prelude::*;
use hdk3::prelude::Path;
mod entries;
pub use entries::{
    profile::{
        ProfileEntry,
        ProfileInput,
        ProfileOutput,
    },
    profile::*
};

// ENTRY DEF DECLARATION
entry_defs![
    Path::entry_def(),
    ProfileEntry::entry_def()
];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

