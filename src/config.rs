extern crate serde_json;

use std::collections::hash_map::HashMap;
use self::serde_json::Value;

use std::fs::File;
use std::io::Read;

pub struct Config {
    id: i32,
    matches: HashMap<String, String>,
}

impl Config {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn matches(&self) -> &HashMap<String, String> {
        &self.matches
    }
}

pub fn load_conf() -> Config {
    let mut file = File::open("config.json").expect("Couldn't access config.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read config.json");

    let mut data: Value = serde_json::from_str(contents.as_str())
                                     .expect("Couldn't parse json data");
    let id = data["id"].as_i64().expect("Expected id to be a number") as i32;
    let mut matches = data["matches"].as_object_mut().expect("Expected matches to be a map");

    let mut matches_iter = matches.iter_mut();
    let mut matches: HashMap<String, String> = HashMap::new();
    while let Some((name, value)) = matches_iter.next() {
        matches.insert(name.to_string(), value.as_str()
                                              .expect("Expected all match values to be strings")
                                              .to_string());
    };

    Config {
        id: id,
        matches: matches,
    }
}
