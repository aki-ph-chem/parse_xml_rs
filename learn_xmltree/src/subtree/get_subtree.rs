use xmltree::Element;
use std::{error, fs};

fn traverse(element: xmltree::Element, e_list: &mut Vec<xmltree::Element>) {
    if element.name == "consts".to_string() {
        println!("Ok");
        e_list.push(element.clone());
    }

    for child in element.children {
        match child {
            xmltree::XMLNode::Element(e) => traverse(e, e_list),
            _ => {},
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error >> {
    let path = "./samples/sub_tree.xml";
    let xml_file = fs::File::open(path)?;
    let elements = Element::parse(xml_file)?;

    let mut e_list = vec![]; 
    traverse(elements, &mut e_list);
    for v in e_list {
        println!("v = {:?}", v);
    }

    Ok(())
}
