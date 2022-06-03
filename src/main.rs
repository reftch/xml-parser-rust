#![forbid(non_camel_case_types)]
#![warn(dead_code)]

use std::{
    fs::{create_dir_all, DirEntry},
    time::Instant,
};

mod utils;
mod xmldocument;
mod xmltag;

pub use xmldocument::XmlDocument;

use utils::traverse_dirs;

static DIR: &str = "xml";
static OUTPUT_DIR: &str = "target/markdown/";

fn main() {
    let start_elapsed = Instant::now();

    // create output directory
    create_dir_all(&OUTPUT_DIR).expect("Error creating output directory");

    let mut entries: Vec<DirEntry> = Vec::new();
    traverse_dirs(DIR, &mut entries);

    for entry in entries {
        println!("Processing file: {:?}", entry.file_name());
        let xml_document = XmlDocument::new(entry.path().display().to_string(), false);
        let rows = xml_document.parse();

        // save file
        utils::write_file(
            String::from(OUTPUT_DIR) + entry.file_name().to_str().unwrap(),
            rows,
        );
    }

    println!("Time elapsed: {:?} ms", start_elapsed.elapsed().as_millis());
}
