#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]

//crates
extern crate inotify;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate blake2;

//imports
use trackercore::read_file;
use trackercore::tracker::Watchlist;
use trackercore::tracker::WatchManager;
use std::io;
use std::thread;
use std::sync::mpsc;

//consts
const SETTINGS: &str = "settings.json";

mod trackercore;

#[derive(Serialize, Deserialize)]
struct SettingsData {
    database: String,
}

fn init() -> Result<SettingsData, io::Error> {
    let settings_file: String = read_file(SETTINGS)?;

    let settings: SettingsData = serde_json::from_str(&settings_file)?;

    Ok(settings)
}

fn main() {

    let settings = init().expect("Couldn't get settings data from file.");

    let db: String = read_file(&settings.database).expect("Couldn't get watchlist from file."); //maybe move this into watcher?
    
    let mut watchlist = Watchlist::new(&settings.database, &db).expect("Error building watchlist");

    let mut watchmanager = WatchManager::new(watchlist).expect("Failed to init watchmanager");

    watchmanager.initialize();
}