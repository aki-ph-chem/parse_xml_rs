use std::collections::HashMap;

fn parse_dict<T>(dict: &HashMap<String, String>) -> Result<(String, T), &'static str> 
where T: std::str::FromStr + Default 
,<T as std::str::FromStr>::Err: std::fmt::Display 
{
    let mut name = String::new();
    let mut value:T = Default::default();
    for (key, v) in dict {
        match key.as_str() {
            "Name" => {name = v.clone();},
            "Value" => {
                value = match (*v).parse::<T>() {
                    Ok(x) => x,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    },
                };
            },
            _ => {
                return Err("Error: key must be 'Name' or 'Value'");
            },
        }
    }
    Ok((name, value))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut const_a = HashMap::new(); 
    const_a.insert("Name".to_string(), "A".to_string());
    const_a.insert("Value".to_string(), "12".to_string());
    let (name, value) = parse_dict::<i32>(&const_a).unwrap();
    println!("(name, value) = ({}, {})", name, value);

    Ok(())
}
