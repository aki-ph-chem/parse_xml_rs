# クレート xmltreeのメモ

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
