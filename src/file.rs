use crate::File;

use std::io::{self, Read};

pub fn read_username_from_file(file: String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(file)?.read_to_string(&mut username)?;
    Ok(username)
}
