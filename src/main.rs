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

    let settings = SettingsData::from_file(SETTINGS).unwrap();

    let watcher = Watcher::new(&settings.database)
        .expect("Error creating watchlist struct");
    /*
   
    let cfgfiles_iterator = cfgfiles.iter();
    
    let mut inotifier = Inotify::init()
        .expect("Failed to init inotify.");

    let mut watches = HashMap::new();

    for file in cfgfiles_iterator {

        watches.insert(&file.filepath, inotifier.add_watch(Path::new(&file.filepath), watch_mask::CLOSE_WRITE)
            .expect("Failed to add watch."));
    }

    //for (filepath, watch) in watches {
    //    inotifier.rm_watch(watch);
    //}

    let mut buffer = [0u8; 4096];

    //loop {
    //    let events = inotifier.read_events_blocking(&mut buffer)
    //        .expect("Failed to read events.");

    //        for event in events {
    //            println!("{:?}", event);
    //        }
    //}

    let j = serde_json::to_string(&cfgfiles)
        .expect("Failed to serialize j");

    if write_file("/home/arsalan/codes/configger-rs/db.json", &j)
        .expect("Error writing db file")
         {

         }
*/
    //ser-de works for us rn

    //inotify works TODO: change MODIFY to WRITE_CLOSE or equiv


    
}