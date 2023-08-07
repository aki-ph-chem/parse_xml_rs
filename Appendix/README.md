# Appendix

### 1 debug\_print.rs: 拡張されれたデバッグプリント

Rustでは以下のように`println!()`マクロで`Debug`属性が実装されている値をprintすることができる。

```Rust
#[derive(Debug)]
struct Hoge {
    value_a: i32,
    value_b: String,
}

let hoge = Hoge{value_a: 1, value_b: "hello".to_string()};
println!("hoge = {:?}", hoge); 

// hoge = Hoge { value_a: 1, value_b: "hello" }
```

しかし、ネストされた値ではprintされた結果が見にくい。
これをもう少し見やすくする方法として`println!()`マクロで`{:?}`としていた部分を`{:#?}`とするとインデントが付きでprintされる

```Rust
#[derive(Debug)]
struct Hoge {
    value_a: i32,
    value_b: Vec<i32>,
}

let hoge = Hoge{value_a: 1, value_b: vec![1, 2, 3]};
println!("hoge = {:#?}", hoge); 

// hoge = Hoge {
//     value_a: 1,
//     value_b: [
//         1,
//         2,
//         3,
//     ],
// }
```
### 2 filter.rs
### 3 fileter\_map.rs
### 4 find.rs
### 5 find\_map.rs

mapが付かない`find()`,`filter()`は`bool`型を返すクロージャーを適用させる。

一方、mapの付く`find_map()`,`filter_map()`は`Option<T>`型の値を返すクロージャーを適用させる。

### 6 hash\_map.rs

```text
{"name": <parameter name>, "value": <value as string>}
```
となっている`HashMap<String, String>`を構造体に変換する。

### 7 hash\_map\_generic.rs

`HashMap<String, String>` の値が

```text
{
"Name": <name of parameter as String>
"Value": <value as String>
}
```

となっているときにこれをタプル

```text
(String,T) = (<name of parameter as String>, value as number)
```

と変換する

### 8. hash\_set.rs

集合型である`HashSet<String>`のメモ
