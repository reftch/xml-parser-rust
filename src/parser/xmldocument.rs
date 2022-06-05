use std::fs::File;
use std::io::BufReader;

use string_builder::Builder;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};
use xml::ParserConfig;

use crate::config::entities::Entities;

use super::xmltag::XmlTag;

pub struct XmlDocument {
    source: File,
    entities: Entities,
    debug: bool,
}

impl XmlDocument {
    pub fn new(source: String, entities: Entities, debug: bool) -> Self {
        Self {
            source: File::open(source).expect("Error opening file"),
            entities,
            debug,
        }
    }

    pub fn parse(&self) -> String {
        let mut builder = Builder::default();

        let file = BufReader::new(&self.source);
        let parser = EventReader::new_with_config(file, self.get_config());

        let mut depth = 0;
        let mut parent_tag = XmlTag::Unknown;
        let mut tag = XmlTag::Unknown;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
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
                        builder.append(format!(
                            "{}{}",
                            XmlDocument::get_list_item(&tag, &parent_tag),
                            data.trim()
                        ));
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
        if self.debug {
            println!("{}{}{}", XmlDocument::indent(*depth), sign, data);
        }
    }

    fn get_config(&self) -> ParserConfig {
        let mut config = ParserConfig::new();
        for (k, v) in self.entities.entities.iter() {
            config = config.add_entity(k, v);
        }
        config
    }

    pub fn indent(size: usize) -> String {
        const INDENT: &str = "    ";
        (0..size)
            .map(|_| INDENT)
            .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
    }

    fn start_element(name: &OwnedName, tag: &XmlTag, attributes: &[OwnedAttribute]) -> String {
        let xml_tag = XmlTag::new(name.local_name.as_str());
        match xml_tag {
            XmlTag::Title => XmlDocument::get_title(tag),
            XmlTag::Paragraph => XmlDocument::get_paragraph(tag),
            XmlTag::ImageData => XmlDocument::get_image(attributes),
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
        let expression = if tag == &XmlTag::Sect1 {
            "\n### "
        } else {
            "## "
        };
        String::from(expression)
    }

    fn get_paragraph(tag: &XmlTag) -> String {
        let expression = if tag != &XmlTag::ListItem { "\n" } else { "" };
        String::from(expression)
    }

    fn get_list_item(tag: &XmlTag, parent_tag: &XmlTag) -> String {
        let expression = if parent_tag == &XmlTag::ListItem && tag == &XmlTag::Paragraph {
            "+ "
        } else {
            ""
        };
        String::from(expression)
    }

    fn get_image(attributes: &[OwnedAttribute]) -> String {
        let attribute = &attributes.get(0);
        match attribute {
            Some(v) => format!("\n![An image](/{})", &v.value),
            None => String::from(""),
        }
    }
}

#[cfg(test)]
#[path = "../_tests/xmldocument.rs"]
mod tests;
