extern crate inotify;
extern crate serde_json;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use inotify::WatchDescriptor;
use serde_json::Value;

fn bootstrap() { //not implemented yet

}

pub fn read_file(filename: &str) -> Result<String, io::Error> {

    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
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

pub struct Database {
    database_file: String,
    pub data: String,
}

impl Database {
    pub fn initialize(db_file: &str) -> Result<Database, io::Error> {
        let data_from_file: String = read_file(&db_file)?;

        let db = Database {
            database_file: String::from("db_file"),
            data: data_from_file,
        };
        Ok(db)
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