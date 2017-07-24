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

struct Watchlist {
    configfile: String,
    watchd: WatchDescriptor,
}

pub struct Watcher {
    json_file: String,
    json: String,
    filelist: Vec<ConfigFile>,
    notifier: Inotify,
    watchlist: Option<Vec<Watchlist>>,

}

impl Watcher{
    pub fn new(json_file: &str) -> Result<Watcher, io::Error> {
        
        let json: String = read_file(&json_file)?;

        let filelist: Vec<ConfigFile> = serde_json::from_str(&json)?;
        
        let mut notifier = Inotify::init()?;

        let mut wl = Watcher {
            json_file: String::from(json_file),
            json,
            filelist,
            notifier,
            watchlist: None,
        };

        wl.init();

        Ok(wl)
    }

    fn init(&mut self) {
        
        let mut watches: Vec<Watchlist> = Vec::new();

        for file in &self.filelist {

            let thisfile_path = String::from(&file.filepath[..]); //hideous
            let this_wd = self.notifier.add_watch(Path::new(&file.filepath), watch_mask::CLOSE_WRITE).unwrap();
            let this_watch = Watchlist {configfile: thisfile_path, watchd: this_wd};
            watches.push(this_watch);
        }

        self.watchlist = Some(watches);
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