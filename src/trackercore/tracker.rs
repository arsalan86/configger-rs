extern crate inotify;
extern crate serde_json;
extern crate blake2;

use std::io;
use trackercore::read_file;
use inotify::{
    event_mask,
    watch_mask,
    Inotify,
    WatchDescriptor,
};
use std::collections::HashMap;
use std::path::Path;
use std::env;
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

pub struct Watcher {
    json_file: String,
    json: String,
    filelist: Vec<ConfigFile>,
    notifier: Inotify,
    watchlist: HashMap<WatchDescriptor,String>,

}

impl Watcher{

    pub fn new(json_file: &str) -> Result<Watcher, io::Error> {
        
        let json: String = read_file(&json_file)?;

        let filelist: Vec<ConfigFile> = serde_json::from_str(&json)?;
        
        let mut notifier = Inotify::init()?;

        let mut watchlist = HashMap::new();

        for file in &filelist {
            let this_filepath = file.filepath.clone();
            let this_wd = notifier.add_watch(Path::new(&file.filepath), watch_mask::CLOSE_WRITE,).unwrap();
            watchlist.insert(this_wd, this_filepath);
        }

        let mut wl = Watcher {
            json_file: String::from(json_file),
            json,
            filelist,
            notifier,
            watchlist,
        };

        Ok(wl)
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

    pub fn get_events(&mut self) {
        //needs to run in a thread?
        let mut buffer = [0u8; 4096];

        let events = self.notifier.read_events_blocking(&mut buffer)
            .expect("Failed to read events.");

        for event in events {
            match self.watchlist.get(&event.wd) {
                Some(x) => println!("{}", x),
                None => println!("None"),
            }
        }
    }
}