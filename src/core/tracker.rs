extern crate inotify;
extern crate serde_json;
extern crate blake2;

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
use std::thread;
use blake2::{Blake2b, Digest};

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub filepath: String,
    pub blake2hash: String,
    pub comment: String,
}

impl ConfigFile {
    pub fn get_hash(&self) -> Result<String, io::Error> {

        let data = read_file(&self.filepath)?;

        let mut hasher = Blake2b::default();

        hasher.input(&data.into_bytes());

        let output = String::from(format!("{:x}", hasher.result()));

        Ok(output)
    }

    pub fn check_hash(&self, hash: String) -> bool {
       let oldhash = &self.blake2hash;
       let newhash = self.get_hash().unwrap();
       oldhash == &newhash
    }

    pub fn get_contents(&self) -> String {
        read_file(&self.filepath).unwrap()

    }

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

            let thisfile_path = file.filepath.clone();
            let this_wd = self.notifier.add_watch(Path::new(&file.filepath), watch_mask::CLOSE_WRITE).unwrap();
            let this_watch = Watchlist {configfile: thisfile_path, watchd: this_wd};
            file.get_hash();
            watches.push(this_watch);
        }

        self.watchlist = Some(watches);
    }

    pub fn add_file(&self, filepath: &str) {
        //check whether file already exists in db, if not, add and then write db to file
        //add a watcher/inotifier for file
    }

    pub fn drop_file(&self, filepath: &str) {
        //check if a file exists in db, and then delete that record
        //remove watcher/inotifier for file
    }

    pub fn write_data(&self) {
        //write the json vector to disk as a json file
    }

    pub fn start(&self) {
        //start the tracker thread
    }

    pub fn stop(&self) {
        //stop the tracker thread
    }
}