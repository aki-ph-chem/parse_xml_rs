// xmlファイルを読み込んでパースする
use xmltree::Element;
use std::{error, fs};

fn main() -> Result<(), Box<dyn error::Error >> {
    let path = "./samples/simple.xml";
    let xml_file = fs::File::open(path)?;
    let elements = Element::parse(xml_file)?;

    if let Some(element_child) = elements.get_child("child") {
        println!("value of element 'child': {:?}", element_child);
    } 

    Ok(())
}
