# xml_parse
Parsing XML, and retrieving specific text or attribute.

* **Baseline**: Simple XML with just one tag, no attributes.
* **Attributes**: 1 tag with 1000 attributes.
* **Serial**: 1000 tags, opening and closing, opening and closing. No nesting.
* **Nested**: 1000 nested entries, each inside of the other.

| | baseline | serial | attribute | nested |
| --- | --- | --- | --- | --- |
| **[dummy_xml](https://crates.io/crates/dummy_xml)** | *0.24* | 138.741 | *194.257* | 142.085 |
| **[quick_xml](https://crates.io/crates/quick_xml)** | 0.334 | *90.26* | 3135.042 | *51.507* |
| **[roxmltree](https://crates.io/crates/roxmltree)** | 0.71 | 264.605 | 2252.596 | 3758.354 |
| **[xmltree](https://crates.io/crates/xmltree)** | 3.819 | 2330.22 | 4273.626 | - |
| **[treexml](https://crates.io/crates/treexml)** | 4.014 | 2328.218 | 4310.757 | 18102.08 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    dummy_xml = "0.1.6"          # Fast Non-validating XML DOM parser.
    quick-xml = "0.13.1"          # High performance xml reader and writer
    roxmltree = "0.4.0"    # Represent an XML as a read-only tree.
    xmltree = "0.8.0"      # Parse an XML file into a simple tree-like structure
    treexml = "0.7.0"      # An XML tree library for Rust

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`