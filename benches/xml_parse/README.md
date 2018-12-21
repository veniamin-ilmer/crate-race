# xml_parse
Parsing XML, and retrieving specific text or attribute.

* **Baseline**: Simple XML with just one tag, no attributes.
* **Attributes**: 1 tag with 1000 attributes.
* **Serial**: 1000 tags, opening and closing, opening and closing. No nesting.
* **Nested**: 1000 nested entries, each inside of the other.

| | baseline | serial | attribute | nested |
| --- | --- | --- | --- | --- |
| **[dummy_xml](https://crates.io/crates/dummy_xml)** | *0.237* | 140.818 | *191.43* | 141.253 |
| **[quick_xml](https://crates.io/crates/quick_xml)** | 0.321 | *89.997* | 3011.394 | *50.655* |
| **[roxmltree](https://crates.io/crates/roxmltree)** | 0.727 | 263.457 | 2253.709 | 3754.625 |
| **[xmltree](https://crates.io/crates/xmltree)** | 3.669 | 2266.988 | 4250.665 | - |
| **[treexml](https://crates.io/crates/treexml)** | 4.029 | 2389.568 | 4271.627 | 18026.861 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    dummy_xml = "0.1.6"          # Fast Non-validating XML DOM parser.
    quick-xml = "0.13.1"          # High performance xml reader and writer
    roxmltree = "0.4.0"    # Represent an XML as a read-only tree.
    xmltree = "0.8.0"      # Parse an XML file into a simple tree-like structure
    treexml = "0.7.0"      # An XML tree library for Rust

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`