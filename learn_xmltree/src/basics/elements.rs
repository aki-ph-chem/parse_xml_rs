// 要素が複数ある場合
use xmltree::Element;
use std::{error, fs};

fn main() -> Result<(), Box<dyn error::Error >> {
    let path = "./samples/elements.xml";
    let xml_file = fs::File::open(path)?;
    let elements = Element::parse(xml_file)?;
    
    let children = ["hoge", "fuga", "piyo"];
    for child in children {
        if let Some(element_child) = elements.get_child(child) {
            println!("value of element 'child': {:?}", element_child);
        } 
    }

    Ok(())
}
