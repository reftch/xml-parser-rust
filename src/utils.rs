use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::{fs::File, io::Write};

pub fn get_source_filename(str: &String) -> &str {
    if str.contains(".xml") {
        &str[0..str.len() - 4]
    } else {
        ""
    }
}

pub fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

pub fn write_file(source: String, rows: String) {
    let file_name = get_source_filename(&source);
    let mut destination = File::create(file_name.to_owned() + ".md").unwrap();
    destination
        .write(rows.as_bytes())
        .expect("Error writing to file");
}

pub fn traverse_dirs(dir: &str, files: &mut Vec<DirEntry>) {
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    traverse_dirs(&entry.path().display().to_string(), files);
                } else {
                    if entry.path().extension().and_then(OsStr::to_str) == Some("xml") {
                        files.push(entry);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_source_filename() {
        assert_eq!(get_source_filename(&String::from("test.xml")), "test");
        assert_eq!(get_source_filename(&String::from("test")), "");
    }

    #[test]
    fn test_traverse_dirs() {
        let mut entries: Vec<DirEntry> = Vec::new();
        traverse_dirs(".", &mut entries);
        assert!(entries.len() > 0);

        let mut entries: Vec<DirEntry> = Vec::new();
        traverse_dirs("", &mut entries);
        assert!(entries.len() == 0);
    }
}
