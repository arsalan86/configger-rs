use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

pub mod tracker;

pub fn read_file(filename: &str) -> Result<String, io::Error> {

    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
    //TODO helpful reminder to check for permissions if read fails
}

pub fn write_file(filename: &str, data: &str) -> Result<bool, io::Error> {

    //should check for overwrite?
    let mut file = OpenOptions::new().write(true).open(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(true)
}