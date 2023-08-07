# クレート xmltreeのメモ

参考: https://github.com/eminence/xmltree-rs

公式のドキュメントを読んでもイマイチだったのでソースコードを読んだときのメモ。

## struct

### 1 struct xmltree::Element

基本的な構造体である`xmltree::Element`について

定義
```Rust
pub struct Element {
    pub prefix: Option<String>,
    pub namespace: Option<String>,
    pub namespaces: Option<Namespace>,
    pub name: String,
    pub attributes: AttributeMap<String, String>,
    pub children: Vec<XMLNode>,
}
```

`AttributeMap<String, String>`は`std::collections::HashMap<String, String>`のaliasである。

### 2 enum xmltree::XMLNode

子ノードを表現する`xmltree::XMLNode`について

定義
```Rust
pub enum XMLNode {
    Element(Element),
    Comment(String),
    CData(String),
    Text(String),
    ProcessingInstruction(String, Option<String>),
}
```

### 3 各種メソッド

1. `get_child()`

`get_child()`は以下で定義されている。

```Rust
impl Element {
    pub fn get_child<P: ElementPredicate>(&self, k: P) -> Option<&Element> {
        self.children
            .iter()
            .filter_map(|e| match e {
                    XMLNode::Element(elem) => Some(elem),
                    _ => None,
                    })
        .find(|e| k.match_element(e))
    }
} 
```

構造体`Element`のフィールド`children`は`Vcc<XMLNode>`であるので`filter_mpa()`までの処理で、`Vec<Element>`へのイテレータが得られる。

ここから先でポイントとなるのがトレイト`ElementPredicate`で以下で定義される。

```Rust
pub trait ElementPredicate {
    fn match_element(&self, e: &Element) -> bool;
}
```

このトレイトを実装するにはメソッド`match_element()`を実装する必要がある。
まず要素が一個のタプル型`(TN,)`に対する実装を見てみよう。

```Rust
impl<TN> ElementPredicate for (TN,)
where
    String: PartialEq<TN>,
{
    fn match_element(&self, e: &Element) -> bool {
        e.name == self.0
    }
}
```

whrere句を見ると型`TN`には`String`型と比較可能であることを要求していて、
処理では構造体`Element`のフィールド`name`とタプルの要素を比較して`bool`値を返している。

トレイト`ElementPredicate`は`(TN,)`以外にも`&str`,`Cow<str>`,`String`,`(TN, NS)`
にも実装されている。

`get_child()`に戻ると最後の`find()`で引数`k`に含まれる文字列が`Element.name`と一致する場合はそれを返すように処理が書かれている。

`get_child()`は上の定義を見ると分かる通り`Element`の`children`フィールドのみを探索するので、ネストされたタグを見つけることはできない。
それを行いたいならば、再帰関数などでトラバースする処理を自分で実装する必要がある。
