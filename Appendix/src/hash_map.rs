use std::collections::HashMap;

#[derive(Debug)]
struct Consts {
    const_a: i32,
    const_b: i32,
    const_c: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut const_a = HashMap::new(); 
    const_a.insert("name".to_string(), "A".to_string());
    const_a.insert("value".to_string(), "12".to_string());

    let mut const_b = HashMap::new(); 
    const_b.insert("value".to_string(), "24".to_string());
    const_b.insert("name".to_string(), "B".to_string());

    let mut const_c = HashMap::new(); 
    const_c.insert("name".to_string(), "C".to_string());
    const_c.insert("value".to_string(), "35".to_string());

    let array_const: Vec<HashMap<String, String>> = vec![
        const_a, const_b ,const_c
    ];


    let consts = {
        let mut consts = Consts{const_a: 0, const_b: 0, const_c: 0};
        for const_map in array_const {
            let (k, v) = {
                let mut k = String::new();
                let mut v = 0;
                for (key, value) in const_map {
                    match key.as_str() {
                        "name" => {k = value.clone();},
                        "value" => {v = value.parse::<i32>().unwrap();},
                        _ => {return  Err("Error: key name should be 'name' or 'value'".into());},
                    }
                }
                (k, v)
            };
            match k.as_str() {
                "A" => {consts.const_a = v;},
                "B" => {consts.const_b = v;},
                "C" => {consts.const_c = v;},
                _ => {return Err("Error: const name should be 'A' or 'B' or 'C'".into());},
            }
        }
        consts
    };

    println!("consts = {:#?}", consts);

    Ok(())
}
