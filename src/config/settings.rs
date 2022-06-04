use config::{Config, File};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    sources: String,
    destination: String,
}

impl Settings {
    pub fn new() -> HashMap<String, String> {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            .build()
            .unwrap();

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("sources: {:?}", s.get_string("sources"));
        println!("destination: {:?}", s.get_string("destination"));

        // Deserialize (and thus freeze) the entire configuration as
        s.try_deserialize::<HashMap<String, String>>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let settings = Settings::new();
        assert_eq!(settings.len(), 3);
        assert_eq!(settings.get("debug").unwrap() == "true", false);
        assert_eq!(settings.get("sources").unwrap(), "xml");
        assert_eq!(settings.get("destination").unwrap(), "markdown/");
    }

}

