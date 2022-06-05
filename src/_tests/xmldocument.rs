use crate::{config::entities::Entities, parser::xmldocument::XmlDocument};

#[test]
fn test_parse_primary_title() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test1.xml"), entities, false);
    assert_eq!(xml_document.parse(), "## Test\n");
}

#[test]
fn test_parse_secondary_title() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test2.xml"), entities, false);
    assert_eq!(xml_document.parse(), "\n### Test\n");
}

#[test]
fn test_parse_paragraph_in_items() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test4.xml"), entities, false);
    assert_eq!(xml_document.parse(), "+ Test\n");
}

#[test]
fn test_parse_guimenu() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test6.xml"), entities, false);
    assert_eq!(xml_document.parse(), " *Test* ");
}

#[test]
fn test_parse_guisubmenu() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test7.xml"), entities, false);
    assert_eq!(xml_document.parse(), " *-> Menu1*  *-> Menu2* ");
}

#[test]
fn test_parse_note() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test8.xml"), entities, false);
    assert_eq!(xml_document.parse(), "\n::: tip\nTest\n:::\n");
}

#[test]
fn test_parse_productname() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test9.xml"), entities, false);
    assert_eq!(xml_document.parse(), " **Test** ");
}

#[test]
fn test_parse_image_data() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test10.xml"), entities, false);
    assert_eq!(
        xml_document.parse(),
        "Image Test\n![An image](/images/test.png)"
    );
}

#[test]
fn test_parse_empty_image_data() {
    let entities = Entities {
        ..Default::default()
    };
    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test11.xml"), entities, false);
    assert_eq!(xml_document.parse(), "");
}

#[test]
fn test_process_xml_entities() {
    let mut entities = Entities {
        ..Default::default()
    };
    entities
        .entities
        .insert("spt".to_owned(), "Some Predicate Title".to_owned());
    entities
        .entities
        .insert("ats".to_owned(), "another test string".to_owned());

    let xml_document =
        XmlDocument::new(String::from("src/_tests/documents/test12.xml"), entities, false);
    assert_eq!(
        xml_document.parse(),
        "## Test **Some Predicate Title** , another test string\n"
    );
}
