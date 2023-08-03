use xmltree::Element;
use std::{error, fs};

fn main() -> Result<(), Box<dyn error::Error >> {
    let path = "./samples/nested.xml";
    let xml_file = fs::File::open(path)?;
    let elements = Element::parse(xml_file)?;

    if let Some(element_child) = elements.get_child("hoges") {
        println!("value of element 'child': {:?}", element_child);
    } 

    Ok(())
}
