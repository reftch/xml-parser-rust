use config::{Config, File};
use std::collections::HashMap;

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Entities {
   pub entities: HashMap<String, String>,
}

impl Entities {
    pub fn new() -> Self {
        let s = Config::builder()
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/entities").required(false))
            .build()
            .unwrap();

        // Deserialize (and thus freeze) the entire configuration as
        Entities { entities: s.try_deserialize::<HashMap<String, String>>().unwrap() }
    }

    pub fn clone(&self) -> Self {
        Entities { entities: self.entities.clone() }
    }
}
