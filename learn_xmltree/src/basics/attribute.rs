// xmlの属性値を取り出す
use xmltree::Element;
use std::{error, fs};

trait Show {
    fn show(&self);
}

impl Show for xmltree::Element {
    fn show(&self) {
        println!("name: {}", self.name);
        for v in &self.attributes{
            println!("attribue: ({},{})", v.0, v.1);
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error >> {
    let path = "./samples/attribute.xml";
    let xml_file = fs::File::open(path)?;
    let elements = Element::parse(xml_file)?;

    let children = ["hoge", "fuga", "piyo"];
    for child in children {
        if let Some(element_child) = elements.get_child(child) {
            element_child.show();
        } 
    }
    Ok(())
}
