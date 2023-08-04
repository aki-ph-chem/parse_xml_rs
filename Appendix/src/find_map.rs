fn main() {
    let array = vec![1, 7, 5, 11, 2, 7, 3, 8];
    // find_map
    let over_10 = array
        .iter()
        .find_map(|x| {
            if *x > 10 {
               Some(*x) 
            } else {
                None
            }
        });

    if let Some(num) = over_10 {
        println!("element over 10 in 'array' is {}", num);
    }
    
    let array = vec![6, 7, 5, 11, 9, 7, 3, 8];
    // find_map
    let equal_11 = array
        .iter()
        .enumerate()
        .find_map(|(index, value)| {
            if *value == 11 {
                Some((index, value))
            } else {
                None
            }
        });

    if let Some(t) = equal_11 {
        println!("array[{}] = {}", t.0, t.1);
    }
}
