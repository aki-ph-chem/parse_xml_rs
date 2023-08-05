use std::collections::HashSet;

fn main() {
    let mut books: HashSet<String> = HashSet::new();

    let books_array = ["Kato", "Suzuki", "Kato", "Oni", "Matumoto", "Suzuki"]
        .iter()
        .map(|s| s.to_string());
    for s in books_array {
        books.insert(s);
    }

    println!("books: {:#?}", books);

    if books.contains("Kato") {
        println!("'book' contain 'Kato'");
    } else {
        println!("'book' not contain 'Kato'");
    }

    if books.contains("Iwabuchi") {
        println!("'book' contain 'Iwabuchi'");
    } else {
        println!("'book' not contain 'Iwabuchi'");
    }
}
