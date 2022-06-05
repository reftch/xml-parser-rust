use config::{Config, File};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(unused)]
pub struct Settings {
    pub settings: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Self {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            .build()
            .unwrap();

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("sources: {:?}", s.get_string("sources"));
        println!("destination: {:?}", s.get_string("destination"));

        Settings { settings: s.try_deserialize::<HashMap<String, String>>().unwrap() }
    }
}

#[cfg(test)]
#[path = "../_tests/settings.rs"]
mod tests;

