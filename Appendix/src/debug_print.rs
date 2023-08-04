// ネストされた複雑な構造体を考える
#[derive(Debug)]
struct Hoge {
    value_a: i32,
    value_b: Vec<i32>,
}

#[derive(Debug)]
struct Fuga {
    num: i32,
    hoge_1: Hoge,
    hoge_2: Hoge,
}

fn main() {
    let fuga = Fuga{
        num: 5,
        hoge_1: Hoge{
            value_a: 123,
            value_b: vec![1,2,3]
        },
        hoge_2: Hoge{
            value_a: 456,
            value_b: vec![4,5,6]
        },
    };

    // 普通のdebug print
    println!("fuga: {:?}", fuga);
    // 拡張されたdebug print
    println!("fuga: {:#?}", fuga);
}
