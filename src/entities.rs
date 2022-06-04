use config::{Config, File};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(unused)]
pub struct Entities {
    list: Vec<String>,
}

impl Entities {
    pub fn new() -> HashMap<String, String> {
        let s = Config::builder()
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/entities").required(false))
            .build()
            .unwrap();

        // Now that we're done, let's access our configuration

        // Deserialize (and thus freeze) the entire configuration as
        s.try_deserialize::<HashMap<String, String>>().unwrap()
    }
}
