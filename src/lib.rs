use std::str::FromStr;

use xmltree::Element;

pub enum Direction {
    Ltr,
    Rtl,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ltr" => Ok(Direction::Ltr),
            "rtl" => Ok(Direction::Rtl),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Id(Option<String>);

impl Id {
    pub fn extract(element: &Element) -> Id {
        match element.attributes.get("id") {
            Some(s) => Id(Some(s.to_owned())),
            None => Id(None),
        }
    }
}

#[derive(Debug)]
pub struct XmlLang(String);

pub struct TitleElement {
    pub id: Id,
    pub dir: Option<Direction>,
    pub lang: Option<XmlLang>,
}

// TODO; enum with struct is efficient ?
pub enum Metadata {
    Title(TitleElement),
    Invalid,
}

impl Metadata {
    pub fn from(elem: Element) -> Option<Self> {
        match (&elem.namespace, &*elem.name) {
            (Some(ns), "title") => {
                if ns == "dc" {
                    let id = Id::extract(&elem);
                    let dir = match elem.attributes.get("dir") {
                        Some(dir) => Direction::from_str(dir).ok(),
                        None => None,
                    };
                    Some(Metadata::Title(TitleElement {
                        id,
                        dir,
                        lang: None,
                    }))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut title_elem =
            Element::parse("<title id='foo' xml:lang='ja' dir='ltr'>タイトル</title>".as_bytes())
                .unwrap();
        title_elem.namespace = Some("dc".to_string());
        let parsed = Metadata::from(title_elem).unwrap();
        if let Metadata::Title(t) = parsed {
            assert_eq!(t.id, Id(Some("foo".to_string())));
        }
    }
}
