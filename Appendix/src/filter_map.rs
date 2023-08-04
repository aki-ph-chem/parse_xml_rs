#[derive(Debug)]
enum TopType {
    TypeA(String),
    TypeB(String),
    TypeC(String),
}

fn main() {
    let vec_top_type = vec![
        TopType::TypeA("hoo".to_string()),
        TopType::TypeB("goo".to_string()),
        TopType::TypeA("ooh".to_string()),
        TopType::TypeC("oog".to_string()),
        TopType::TypeA("oho".to_string()),
        TopType::TypeC("ogo".to_string()),
    ];

    let type_a: Vec<&String> = vec_top_type
        .iter()
        .filter_map(|s| match s {
            TopType::TypeA(s) => Some(s),
            _ => None,
        }).collect();

    for s in type_a {
        println!("s = {}", s);
    }

}
