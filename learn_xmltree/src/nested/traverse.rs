use xmltree::Element;
use std::{error, fs};

fn traverse(element: xmltree::Element) {
    println!("name = {}", element.name);

    for child in element.children {
        match child {
            xmltree::XMLNode::Element(e) => traverse(e),
            xmltree::XMLNode::Comment(s) => println!("Comments: {}",s),
            xmltree::XMLNode::CData(s) => println!("CData: {}",s),
            xmltree::XMLNode::Text(s) => println!("Text: {}",s),
            _ => {},
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error >> {
    let path = "./samples/nested.xml";
    let xml_file = fs::File::open(path)?;
    let elements = Element::parse(xml_file)?;
    traverse(elements);

    Ok(())
}
