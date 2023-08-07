fn main() {

    let array = [12, 15, 7, 3, 5, 10];

    let array_over_10: Vec<&i32> = array
        .iter()
        .filter(|x| **x >= 10)
        .collect();

    println!("araay_over_10 = {:#?}", array_over_10);
}
