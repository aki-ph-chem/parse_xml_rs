use xmltree::Element;
use std::{error, fs};

#[derive(Debug)]
struct Parameter{
    state: String,
    const_a: i32,
    const_b: i32,
    const_c: i32,
}

impl Parameter {
    fn new(element: &xmltree::Element) -> Result<Parameter, &'static str> {
        let (_, state) = match element.attributes.iter().next() {
            Some(v) => v,
            None => {return Err("Error: None");},
        };

        let child_child = match element.children.iter().next() {
            Some(v) => {
                match v {
                    xmltree::XMLNode::Element(e) => &e.children,
                    _ => {return Err("Error: Thre is no 'children'");},
                }
            },
            None => {return Err("Error: None");},
        };

        let mut parameter = Parameter{state: state.to_string(), const_a: 0, const_b: 0, const_c: 0};
        for const_map in child_child {
            if let xmltree::XMLNode::Element(element) = const_map {

                let (name, value) = {
                    let mut name = String::new();
                    let mut v = 0;
                    for (key, value) in element.attributes.iter() {
                        match key.as_str() {
                            "name" => {name = value.to_string();},
                            "value" => {v = value.parse::<i32>().unwrap()}, 
                            _ => { return  Err("Error: key name should be 'name' or 'value'")}
                        }
                    }
                    (name, v)
                };

                match name.as_str() {
                    "A" => {parameter.const_a = value},
                    "B" => {parameter.const_b = value},
                    "C" => {parameter.const_c = value},
                    _ => {return Err("Error: const name should be 'A' or 'B' or 'C'")},
                }
            }
        }
        Ok(parameter)
    }
}

fn traverse(element: xmltree::Element, e_list: &mut Vec<xmltree::Element>) {

    if element.name == "state" {
        e_list.push(element.clone());
        return;
    } 

    for child in element.children {
        match child {
            xmltree::XMLNode::Element(e) => traverse(e, e_list),
            _ => {},
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "./samples/sub_tree_attribute.xml";
    let xml_file = fs::File::open(path)?;
    let elements = Element::parse(xml_file)?;

    let mut e_list = vec![]; 
    traverse(elements, &mut e_list);
    let state_ground = Parameter::new(&e_list[0])?;
    let state_excited = Parameter::new(&e_list[1])?;

    println!("state_ground = {:#?}", state_ground);
    println!("state_excited = {:#?}", state_excited);

    Ok(())
}
