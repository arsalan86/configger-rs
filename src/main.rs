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
use core::{
    read_file,
};
use core::tracker::Watcher;
use std::io;
//use std::result;

//consts
//const SETTINGS : &str = "/var/lib/configger/settings.json"; //hardcoded?
const SETTINGS : &str = "/home/arsalan/codes/configger-rs/settings.json"; //use-local


mod core;

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

    watcher.start();
    /*
   
    for (filepath, watch) in watches {
       inotifier.rm_watch(watch);
    }

    let j = serde_json::to_string(&cfgfiles)
        .expect("Failed to serialize j");

    if write_file("/home/arsalan/codes/configger-rs/db.json", &j)
        .expect("Error writing db file")
         {

         }
*/
    //ser-de works for us rn
    
}