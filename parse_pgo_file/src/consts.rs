use xmltree::Element;
use std::{error, fs};

struct Traverse<'a> {
    target: &'a str,
}

impl Traverse<'_> {

    fn traverse(&self, element: xmltree::Element, e_list: &mut Vec<xmltree::Element>) {
        if element.name == self.target  {
            e_list.push(element.clone());
            return;
        } 

        for child in element.children {
            match child {
                xmltree::XMLNode::Element(e) => self.traverse(e, e_list),
                _ => {},
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "samples/pgo.xml";
    let pgo_file = fs::File::open(path)?;
    let elements = Element::parse(pgo_file)?;

    //let tr = Traverse{target: "AsymmetricTop"};
    let tr = Traverse{target: "AsymmetricManifold"};

    let mut mol = vec![]; 
    tr.traverse(elements, &mut mol);
    println!("mol : {:#?}", mol);

    Ok(())
}
