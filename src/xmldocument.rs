use std::fs::File;
use std::io::BufReader;

use string_builder::Builder;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};

use crate::utils::indent;
use crate::xmltag::XmlTag;

pub struct XmlDocument {
    source: File,
    logging: bool,
}

impl XmlDocument {
    pub fn new(source: String, logging: bool) -> Self {
        Self {
            source: File::open(source).expect("Error opening file"),
            logging,
        }
    }

    pub fn parse(&self) -> String {
        let mut builder = Builder::default();

        let file = BufReader::new(&self.source);
        let parser = EventReader::new(file);

        let mut depth = 0;
        let mut parent_tag = XmlTag::Unknown;
        let mut tag = XmlTag::Unknown;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    self.logging(&name.local_name, '+', &depth);
                    depth += 1;

                    builder.append(XmlDocument::start_element(&name, &tag, &attributes));
                    // save parent tag
                    parent_tag = tag;
                    tag = XmlTag::new(&name.local_name);
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    self.logging(&name.local_name, '-', &depth);

                    builder.append(XmlDocument::end_element(name));

                    // clear tag
                    tag = XmlTag::Unknown;
                }
                Ok(XmlEvent::Characters(data)) => {
                    self.logging(data.trim(), ' ', &depth);

                    if tag != XmlTag::Primary && tag != XmlTag::Secondary {
                        builder.append(format!("{}{}", XmlDocument::get_list_item(&tag, &parent_tag), data.trim()));
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        builder.string().unwrap()
    }

    fn logging(&self, data: &str, sign: char, depth: &usize) {
        if self.logging {
            println!("{}{}{}", indent(*depth), sign, data);
        }
    }

    fn start_element(name: &OwnedName, tag: &XmlTag, attributes: &Vec<OwnedAttribute>) -> String {
        let xml_tag = XmlTag::new(name.local_name.as_str());
        match xml_tag {
            XmlTag::Title => XmlDocument::get_title(tag),
            XmlTag::Paragraph => XmlDocument::get_paragraph(tag),
            XmlTag::ImageData => XmlDocument::get_image(&attributes),
            XmlTag::GuiLabel => String::from(" *"),
            XmlTag::GuiMenu => String::from(" *"),
            XmlTag::GuiSubMenu => String::from(" *-> "),
            XmlTag::Note => String::from("\n::: tip"),
            XmlTag::ItemizedList => String::from("\n"),
            XmlTag::ProductName => String::from(" **"),
            _ => String::from(""),
        }
    }

    fn end_element(name: OwnedName) -> &'static str {
        let xml_tag = XmlTag::new(name.local_name.as_str());
        match xml_tag {
            XmlTag::Title => "\n",
            XmlTag::Paragraph => "\n",
            XmlTag::GuiLabel => "* ",
            XmlTag::GuiMenu => "* ",
            XmlTag::GuiSubMenu => "* ",
            XmlTag::Note => ":::\n",
            XmlTag::ItemizedList => "\n",
            XmlTag::ProductName => "** ",
            _ => "",
        }
    }

    fn get_title(tag: &XmlTag) -> String {
        if tag == &XmlTag::Sect1 {
            String::from("\n### ")
        } else {
            String::from("## ")
        }
    }

    fn get_paragraph(tag: &XmlTag) -> String {
        if tag != &XmlTag::ListItem {
            String::from("\n")
        } else {
            String::from("")
        }
    }

    fn get_list_item(tag: &XmlTag, parent_tag: &XmlTag) -> String {
        if parent_tag == &XmlTag::ListItem && tag == &XmlTag::Paragraph {
            String::from("+ ")
        } else {
            String::from("")
        }
    }

    fn get_image(attributes: &Vec<OwnedAttribute>) -> String {
        let attribute = &attributes.get(0);
        match attribute {
            Some(v) => format!("\n![An image](/{})", &v.value),
            None => String::from("")
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_primary_title() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test1.xml"), false);
        assert_eq!(xml_document.parse(), "## Test\n");
    }

    #[test]
    fn test_parse_secondary_title() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test2.xml"), false);
        assert_eq!(xml_document.parse(), "\n### Test\n");
    }

    #[test]
    fn test_parse_paragraph_in_items() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test4.xml"), false);
        assert_eq!(xml_document.parse(), "+ Test\n");
    }

    #[test]
    fn test_parse_guimenu() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test6.xml"), false);
        assert_eq!(xml_document.parse(), " *Test* ");
    }

    #[test]
    fn test_parse_guisubmenu() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test7.xml"), false);
        assert_eq!(xml_document.parse(), " *-> Menu1*  *-> Menu2* ");
    }

    #[test]
    fn test_parse_note() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test8.xml"), false);
        assert_eq!(xml_document.parse(), "\n::: tip\nTest\n:::\n");
    }

    #[test]
    fn test_parse_productname() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test9.xml"), false);
        assert_eq!(xml_document.parse(), " **Test** ");
    }

    #[test]
    fn test_parse_image_data() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test10.xml"), false);
        assert_eq!(xml_document.parse(), "Image Test\n![An image](/images/test.png)");
    }

    #[test]
    fn test_parse_empty_image_data() {
        let xml_document = XmlDocument::new(String::from("tests/documents/test11.xml"), false);
        assert_eq!(xml_document.parse(), "");
    }

}
