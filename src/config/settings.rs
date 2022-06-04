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
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let settings = Settings::new();
        assert_eq!(settings.settings.len(), 3);
        assert_eq!(settings.settings.get("debug").unwrap() == "true", false);
        assert_eq!(settings.settings.get("sources").unwrap(), "xml");
        assert_eq!(settings.settings.get("destination").unwrap(), "markdown/");
    }

}

