// &str型から構成する
use xmltree::Element;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>>{
    let xml_string = r#"
<root>
    <hoge>hoge</hoge>
    <fuga>fuga</fuga>
    <piyo>piyo</piyo>
</root>
       "#;

    let elements = Element::parse(xml_string.as_bytes())?;

    let children = ["hoge", "fuga", "piyo"];
    for child in children {
        if let Some(element_child) = elements.get_child(child) {
            println!("value of element 'child': {:?}", element_child);
        } 
    }
    
    Ok(())
} 
