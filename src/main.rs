#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

//crates
extern crate inotify;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate blake2;

//imports
use trackercore::{
    read_file,
};
use trackercore::tracker::Watcher;
use std::io;
//use std::result;
use std::thread;
use std::sync::mpsc;

//consts
//const SETTINGS : &str = "/var/lib/configger/settings.json"; //hardcoded?
const SETTINGS : &str = "/home/arsalan/codes/configger-rs/settings.json"; //use-local


mod trackercore;

#[derive(Serialize, Deserialize)]
struct SettingsData {
    version: String,
    database: String,
}

impl SettingsData  {
    fn from_file(filename: &str) -> Result<SettingsData, io::Error> {
        
        let settings_file: String = read_file(filename)?;

        let settings: SettingsData = serde_json::from_str(&settings_file)?;

        Ok(settings)

    }
}

fn main() {

    //begin bootstrap

    let settings = SettingsData::from_file(SETTINGS)
        .expect("Couldn't get settings data from file.");

    let mut watcher = Watcher::new(&settings.database)
        .expect("Error creating watchlist struct");

    //let (tx, rx) = mpsc::channel();

    //thread::spawn(move || {

        let xx = watcher.get_events();

    //});
}