use xmltree::Element;
use std::{error, fs};
use std::collections::HashMap;

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

        let mut const_a = 0; 
        let mut const_b = 0; 
        let mut const_c = 0;
        for c in child_child {
            if let xmltree::XMLNode::Element(e) = c {
                let mut name = String::new();
                let mut v = String::new();
                for (key, value) in e.attributes.iter() {
                    if key == "name" {
                        name = value.to_string();
                    } else if key == "value" {
                        v = value.to_string();
                    }
                }
                if name == "A" {
                    const_a = v.parse::<i32>().unwrap();
                } else if name == "B" {
                    const_b = v.parse::<i32>().unwrap();
                } else if name == "C" {
                    const_c = v.parse::<i32>().unwrap();
                }
            }
        }

        Ok(Parameter{state: state.to_string(), const_a, const_b, const_c})
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
