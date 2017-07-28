use std::io;
use trackercore::{
    read_file,
    write_file
};
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
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub filepath: String,
    pub blake2hash: String,
    pub comment: String,

}

impl ConfigFile {
    pub fn get_hash(&mut self) -> Result<String, io::Error> {

        let data = read_file(&self.filepath)?;

        let mut hasher = Blake2b::default();

        hasher.input(&data.into_bytes());

        let output = String::from(format!("{:x}", hasher.result()));

        Ok(output)
    }

    pub fn check_hash_changed(&mut self) -> bool {

        let oldhash = self.blake2hash.clone();

        self.blake2hash = self.get_hash().unwrap();

        oldhash != self.blake2hash
    }

    pub fn get_contents(&self) -> String {

        read_file(&self.filepath).unwrap()

    }

}

// Watcher<watchmanager> should be simple interface between inotify and everything else.
// It should basically accept filenames to add/drop files to the inotify watcher,
// and start/stop the watcher event loop.

pub struct WatchManager {
    notifier: Inotify,
}

impl WatchManager {
    pub fn new() -> Result<WatchManager, io::Error> {

        let mut notifier = Inotify::init()?;

        let mut wm = WatchManager {
            notifier,
        };

        Ok(wm)
    }

    pub fn add_watch(&mut self, filename: &str) -> Result<WatchDescriptor, io::Error> {
        Ok(self.notifier.add_watch(Path::new(filename), watch_mask::CLOSE_WRITE,)?)
    }
    pub fn drop_watch(&mut self, wd: WatchDescriptor) {
        self.notifier.rm_watch(wd);
    }
}

pub struct Watcher {
    json_file: String, //Watchmanager does not need to deal with jsons or data
    json: String, // ditto
    filelist: Vec<ConfigFile>, //nope, it needs to be passed files using an add_watch(method)
    notifier: Inotify,
    watchlist: HashMap<WatchDescriptor,String>, //this can be simplified

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

        // let mut changed_flag = false;

        // for file in wl.filelist {
        //     if file.check_hash_changed() {
        //         changed_flag = true;
        //     }
        // }

        // if changed_flag {
        //     wl.write_data();
        // }

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
        let j = serde_json::to_string(&self.filelist)
            .expect("Failed to serialize j");
        write_file(&self.json_file, &j);
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