extern crate inotify;
extern crate serde_json;

use std::io;
use core::read_file;
use inotify::{
    event_mask,
    watch_mask,
    Inotify,
    WatchDescriptor,
};
use std::collections::HashMap;
use std::path::Path;

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

    //pub fn get_contents(&self) -> String {
    //    let s: String = "Nothing here yet".to_string();
    //    s
    //}

    //pub fn event_trigger(&self) {
    //    ()
    //}

}

struct Watcher {
    configfile: ConfigFile,
    watchd: WatchDescriptor,
}

pub struct Watchlist {
    json_file: String,
    json: String,
    filelist: Vec<ConfigFile>,
    notifier: Inotify,
    watcher: Option<Vec<Watcher>>,

}

impl Watchlist {
    pub fn new(json_file: &str) -> Result<Watchlist, io::Error> {
        
        let json: String = read_file(&json_file)?;

        let filelist: Vec<ConfigFile> = serde_json::from_str(&json)?;
        
        let mut notifier = Inotify::init()?;

        // let mut watches = HashMap::new();

        // for file in filelist_i.iter() {
        //     watches.insert(file, notifier.add_watch(Path::new(&file.filepath), watch_mask::CLOSE_WRITE)?);
        // }

        let wl = Watchlist {
            json_file: String::from(json_file),
            json,
            filelist,
            notifier,
            watcher: None,
        };

        Ok(wl)
    }

    pub fn init(&self) {
        
        //et watchers 
    }
    //pub fn read_data(&self) {

    //}
    //pub fn write_data(&self) {

    //}
    //pub fn add_file(&self) {

    //}
    //pub fn rm_file(&self) {

    //}
}