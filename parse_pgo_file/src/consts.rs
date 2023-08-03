use xmltree::Element;
use std::{error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "samples/pgo.xml";
    let pgo_file = fs::File::open(path)?;
    let elements = Element::parse(pgo_file)?;

    //println!("elements: {:?}", elements);
    let key = "AsymmetricTop";
    let mol = elements.get_child(key);
    println!("mol : {:?}", mol);

    Ok(())
}
