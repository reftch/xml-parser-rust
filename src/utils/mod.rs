use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io::Error;
use std::{fs::File, io::Write};

fn get_source_filename(str: &String) -> Option<&str> {
    if str.contains(".xml") {
        Some(&str[0..str.len() - 4])
    } else {
        None
    }
}

pub fn write_file(source: String, rows: String) -> Result<usize, Error> {
    let file_name = get_source_filename(&source);
    if let Some(file_name) = file_name {
        let mut destination = File::create(file_name.to_owned() + ".md").unwrap();
        destination.write(rows.as_bytes())
    } else {
        panic!("Failed attemp converting non-XML file: {:?}", source);
    }
}

pub fn traverse_dirs(dir: &str, files: &mut Vec<DirEntry>) {
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.into_iter().flatten() {
            let path = entry.path();
            if path.is_dir() {
                traverse_dirs(&entry.path().display().to_string(), files);
            } else if entry.path().extension().and_then(OsStr::to_str) == Some("xml") {
                files.push(entry);
            }
        }
    }
}

#[cfg(test)]
#[path = "../_tests/utils.rs"]
mod tests;
