#![forbid(non_camel_case_types)]
#![warn(dead_code)]

use std::{
    collections::HashMap,
    fs::{create_dir_all, DirEntry},
    time::Instant,
};

mod entities;
mod settings;
mod utils;
mod xmldocument;
mod xmltag;

pub use xmldocument::XmlDocument;

use entities::Entities;
use settings::Settings;
use utils::traverse_dirs;

fn main() {
    let start_elapsed = Instant::now();

    let settings = Settings::new();
    let entities: HashMap<String, String> = Entities::new();

    let debug = settings.get("debug").unwrap() == "true";

    // create output directory
    create_dir_all(settings.get("destination").unwrap()).expect("Error creating output directory");

    let mut entries: Vec<DirEntry> = Vec::new();
    traverse_dirs(settings.get("sources").unwrap(), &mut entries);

    for entry in entries {
        println!("Processing file: {:?}", entry.file_name());
        let xml_document =
            XmlDocument::new(entry.path().display().to_string(), entities.clone(), debug);
        let rows = xml_document.parse();

        // save file
        utils::write_file(
            String::from(settings.get("destination").unwrap())
                + entry.file_name().to_str().unwrap(),
            rows,
        );
    }

    println!("Time elapsed: {:?} ms", start_elapsed.elapsed().as_millis());
}
