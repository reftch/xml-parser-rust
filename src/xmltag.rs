
#[derive(Debug, PartialEq, Eq)]
pub enum XmlTag {
  Title,
  Sect1,
  Paragraph,
  GuiLabel,
  GuiMenu,
  GuiSubMenu,
  Note,
  ListItem,
  ItemizedList,
  ProductName,
  Primary,
  Secondary,
  ImageObject,
  ImageData,
  Unknown,
}

impl XmlTag {
  pub fn new(title: &str) -> XmlTag {
      match title {
          "title" => XmlTag::Title,
          "sect1" => XmlTag::Sect1,
          "para" => XmlTag::Paragraph,
          "guilabel" => XmlTag::GuiLabel,
          "guimenu" => XmlTag::GuiMenu,
          "guisubmenu" => XmlTag::GuiSubMenu,
          "note" => XmlTag::Note,
          "listitem" => XmlTag::ListItem,
          "productname" => XmlTag::ProductName,
          "itemizedlist" => XmlTag::ItemizedList,
          "primary" => XmlTag::Primary,
          "secondary" => XmlTag::Secondary,
          "imageobject" => XmlTag::ImageObject,
          "imagedata" => XmlTag::ImageData,
          _ => XmlTag::Unknown,
      }
  }
}
