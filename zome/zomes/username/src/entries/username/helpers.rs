use hdk::prelude::*;

pub fn path_from_str(string_slice: &str) -> Path {
    let path = Path::from(string_slice);
    // Tats: expect() panics, should we use it?
    path.ensure().expect("Path could not be ensured");
    path
}