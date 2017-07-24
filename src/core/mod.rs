extern crate inotify;
extern crate serde_json;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use inotify::WatchDescriptor;
use serde_json::Value;

pub fn read_file(filename: &str) -> Result<String, io::Error> {

    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(filename: &str, data: &str) -> Result<bool, io::Error> {

    //should check for overwrite?
    let mut file = OpenOptions::new().write(true).open(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(true)
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub filepath: String,
    pub sha1hash: String,
    pub comment: String,
}

impl ConfigFile {
    pub fn get_hash(&self) {
        
    }

    //pub fn check_hash(&self, hash: String) -> bool {
    //    true
    //}
//
    //pub fn get_contents(&self) -> String {
    //    let s: String = "Nothing here yet".to_string();
    //    s
    //}
//
    //pub fn event_trigger(&self) {
    //    ()
    //}

}

pub struct Watchlist {
    json_file: String,
    json: String,
    filelist: Vec<ConfigFile>,
}

impl Watchlist {
    pub fn initialize(json_file: &str) -> Result<Watchlist, io::Error> {
        
        let json: String = read_file(&json_file)?;

        let filelist: Vec<ConfigFile> = serde_json::from_str(&json)?;

        let wl = Watchlist {
            json_file: String::from("json_file"),
            json,
            filelist,
        };

        Ok(wl)
    }
    //pub fn read_data(&self) {
//
    //}
    //pub fn write_data(&self) {
//
    //}
    //pub fn add_file(&self) {
//
    //}
    //pub fn rm_file(&self) {
//
    //}
}