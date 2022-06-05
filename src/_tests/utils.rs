use std::fs::DirEntry;

use crate::utils::{get_source_filename, traverse_dirs};

#[test]
fn test_get_source_filename() {
    assert_eq!(get_source_filename(&String::from("test.xml")), Some("test"));
    assert_eq!(get_source_filename(&String::from("test")), None);
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
