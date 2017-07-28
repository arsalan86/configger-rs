use std::io;
use trackercore::{read_file, write_file};
use inotify::{event_mask, watch_mask, Inotify, WatchDescriptor};
use std::collections::HashMap;
use std::path::Path;
use std::env;
use blake2::{Blake2b, Digest};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    filepath: String,
    blake2hash: String,
    comment: String,
}

impl ConfigFile {
    fn update_hash(&mut self) -> Result<bool, io::Error> {

        let file_data = read_file(&self.filepath)?;
        let mut hasher = Blake2b::default();
        hasher.input(&file_data.into_bytes());
        let newhash = String::from(format!("{:x}", hasher.result()));
        let oldhash = self.blake2hash.clone();
        self.blake2hash = newhash;
        Ok(oldhash != self.blake2hash) //return true if changed
    }

    fn get_contents(&self) -> String {

        read_file(&self.filepath).unwrap()
    }

    fn touch(&self) {
        println!("{} touched", self.filepath);
        //backup and update hash
    }
}

#[derive(Debug)]
pub struct Watchlist {
    database: String,
    filelist: HashMap<String, ConfigFile>,
    watches: HashMap<WatchDescriptor, String>,
}

impl Watchlist {
    pub fn new(database: &str, dataset: &str) -> Result<Watchlist, io::Error> {

        let files: Vec<ConfigFile> = serde_json::from_str(&dataset)?;
        let mut filelist = HashMap::new();
        let mut file_changed_flag = false;
        for mut file in files {
            if file.update_hash().expect("failed to update hash") {
                // make a backup here
                file_changed_flag = true;
            }
            filelist.insert(file.filepath.clone(), file);
        }
        let watches = HashMap::new();
        let mut wl = Watchlist {
            database: String::from(database),
            filelist,
            watches,
        };
        if file_changed_flag {
            wl.write_data();
        }
        Ok(wl)
    }

    fn add_file(&mut self) {}

    fn drop_file(&mut self) {}

    fn write_data(&self) {

        let mut files = Vec::new();

        for (filepath, file) in self.filelist.iter() {
            files.push(file);
        }

        let j = serde_json::to_string(&files).expect("Failed to serialize j");

        write_file(&self.database, &j);
    }
}

pub struct WatchManager {
    notifier: Inotify,
    watchlist: Watchlist,
}

impl WatchManager {
    pub fn new(watchlist: Watchlist) -> Result<WatchManager, io::Error> {

        let mut notifier = Inotify::init()?;
        let mut wm = WatchManager {
            notifier,
            watchlist,
        };
        Ok(wm)
    }

    pub fn initialize(&mut self) {

        self.start();
    }

    pub fn add_watch(&mut self, filename: &str) -> Result<WatchDescriptor, io::Error> {

        Ok(self.notifier.add_watch(
            Path::new(filename),
            watch_mask::CLOSE_WRITE,
        )?)
    }

    pub fn drop_watch(&mut self, wd: WatchDescriptor) {

        self.notifier.rm_watch(wd);
    }

    pub fn start(&mut self) {

        let mut filepaths: Vec<String> = Vec::new();
        for (filepath, file) in &self.watchlist.filelist {
            filepaths.push(filepath.clone());
        }
        for filepath in filepaths {
            let wd = self.add_watch(&filepath).unwrap();
            self.watchlist.watches.insert(wd, filepath.clone());
        }
        //needs to run in a thread?
        let mut buffer = [0u8; 4096];
        let events = self.notifier.read_events_blocking(&mut buffer).expect(
            "Failed to read events.",
        );
        let mut eventpaths: Vec<String> = Vec::new();
        for event in events {
            match self.watchlist.watches.get(&event.wd) {
                Some(filepath) => eventpaths.push(filepath.clone()), //need to map to touched event and update db
                None => println!("None"),
            }
        }
        println!("{:#?}", eventpaths);
        for filepath in eventpaths {
            self.watchlist.filelist.get(&filepath).unwrap().touch();
        }
    }

    pub fn stop(&self) {}
}