use std::{
    fs::File,
    io::{self, Read},
};

#[allow(dead_code)]
pub fn read_username_from_file(file: String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(file)?.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(dead_code)]
pub fn read_username_from_file_via_match(file: String) -> Result<String, io::Error> {
    let mut f = File::open(file)?;
    let mut username = String::new();
    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
