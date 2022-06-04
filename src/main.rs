#![forbid(non_camel_case_types)]
#![warn(dead_code)]

use std::{
    fs::{create_dir_all, DirEntry},
    time::Instant,
};

mod config;
mod parser;
mod utils;

use crate::parser::xmldocument::XmlDocument;
use crate::config::settings::Settings;
use crate::config::entities::Entities;

use utils::traverse_dirs;

fn main() {
    let start_elapsed = Instant::now();

    let settings = Settings::new();
    let entities = Entities::new();

    let debug = settings.settings.get("debug").unwrap() == "true";

    // create output directory
    create_dir_all(settings.settings.get("destination").unwrap()).expect("Error creating output directory");

    let mut entries: Vec<DirEntry> = Vec::new();
    traverse_dirs(settings.settings.get("sources").unwrap(), &mut entries);

    for entry in entries {
        println!("Processing file: {:?}", entry.file_name());
        let xml_document =
            XmlDocument::new(entry.path().display().to_string(), entities.clone(), debug);
        let rows = xml_document.parse();

        // save file
        let result = utils::write_file(
            String::from(settings.settings.get("destination").unwrap())
                + entry.file_name().to_str().unwrap(),
            rows,
        );

        match result {
            Ok(_) => {},
            Err(e) => println!("Error saving file: {:?}", e),
        }
    }

    println!("Time elapsed: {:?} ms", start_elapsed.elapsed().as_millis());
}
