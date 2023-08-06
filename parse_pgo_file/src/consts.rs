use xmltree::Element;
use std::{error, fs};

struct Traverse<'a> {
    target_1: &'a str,
    target_2: &'a str,
}

impl Traverse<'_> {

    fn traverse(&self, element: xmltree::Element, e_list: &mut Vec<xmltree::Element>) {
        if element.name == self.target_1  {
            e_list.push(element.clone());
            return;
        } else if element.name == self.target_2 {
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

#[derive(Debug)]
struct RotationalConsts{
    state: String,
    origin: f64,
    const_a: f64,
    const_b: f64,
    const_c: f64,
}

impl RotationalConsts {
    fn new(element: &xmltree::Element) -> Result<RotationalConsts, &'static str> {
        let state = element.attributes["Name"].clone();

        let child_child = match element.children.iter().next() {
            Some(v) => {
                match v {
                    xmltree::XMLNode::Element(e) => &e.children,
                    _ => {return Err("Error: Thre is no 'children'");},
                }
            },
            None => {return Err("Error: None");},
        };

        let mut parameter = RotationalConsts{
            state: state, origin: 0.0, 
            const_a: 0.0, const_b: 0.0, const_c: 0.0};

        for const_map in child_child {
            if let xmltree::XMLNode::Element(element) = const_map {

                let (name, value) = {
                    let mut name = String::new();
                    let mut value = 0.0;
                    for (key, v) in element.attributes.iter() {
                        match key.as_str() {
                            "Name" => {name = v.to_string();},
                            "Value" => {value = v.parse::<f64>().unwrap()}, 
                            _ => { return  Err("Error: key name should be 'name' or 'value'")}
                        }
                    }
                    (name, value)
                };

                match name.as_str() {
                    "A" => {parameter.const_a = value},
                    "B" => {parameter.const_b = value},
                    "C" => {parameter.const_c = value},
                    "Origin" => {parameter.origin = value},
                    _ => {return Err("Error: const name should be 'A' or 'B' or 'C'")},
                }
            }
        }
        Ok(parameter)
    }
}

#[derive(Debug)]
struct Paremters {
    name: String,
    value: f64,
}

impl Paremters {
    fn new(element: &xmltree::Element) -> Result<Paremters, &'static str> {
        let mut name = String::new();
        let mut value = 0.0;
        for (key, v) in &element.attributes {
            match key.as_str() {
                "Name" => {name = v.to_string();},
                "Value" => {value = v.parse::<f64>().unwrap();},
                _ => {return Err("Error");},
            }
        }
        Ok(Paremters{name, value})
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "samples/pgo.xml";
    let pgo_file = fs::File::open(path)?;
    let elements = Element::parse(pgo_file)?;

    let tr = Traverse{
        target_1: "AsymmetricManifold",
        target_2: "Parameter",
    };

    let mut mol = vec![]; 
    tr.traverse(elements, &mut mol);
    
    let mut consts = vec![];
    let mut paremeters = vec![];
    for m in mol {
        if &m.name == "AsymmetricManifold" {
            if let Ok(v) = RotationalConsts::new(&m) {
                consts.push(v);
            }
        }else{
            if let Ok(v) = Paremters::new(&m) {
                paremeters.push(v);
            }
        } 
    }

    for i in consts {
        println!("i = {:#?}", i);
    }

    for i in paremeters {
        println!("i = {:#?}", i);
    }

    Ok(())
}
