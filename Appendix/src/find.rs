fn main() {
    let array = vec![1, 7, 5, 11, 2, 7, 3, 8];
    // find
    let over_10 = array
        .iter()
        .find(|x| {
            **x > 10
        });

    if let Some(num) = over_10 {
        println!("element over 10 in 'array' is {}", num);
    }

    let array = vec![1, 7, 5, 11, 2, 7, 3, 8];
    // find
    let over_10 = array
        .iter()
        .enumerate()
        .find(|(index, value)| {
            **value == 11
        });

    if let Some(num) = over_10 {
        println!("array[{}] = {}", num.0, num.1);
    }
}
