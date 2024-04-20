use crate::File;

use std::io::{self, Read};

pub fn read_username_from_file(file: String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(file)?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn read_username_from_file_via_match(file: String) -> Result<String, io::Error> {
    let f = File::open(file);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
