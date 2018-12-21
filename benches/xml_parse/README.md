# xml_parse
Parsing XML, and retrieving specific text or attribute.

* **Baseline**: Simple XML with just one tag, no attributes.
* **Attributes**: 1 tag with 1000 attributes.
* **Serial**: 1000 tags, opening and closing, opening and closing. No nesting.
* **Nested**: 1000 nested entries, each inside of the other.

| | baseline | serial | attribute | nested |
| --- | --- | --- | --- | --- |
| **[dummy_xml](https://crates.io/crates/dummy_xml)** | *0.257* | 143.18 | *192.331* | 143.081 |
| **[quick_xml](https://crates.io/crates/quick_xml)** | 0.321 | *98.741* | 2874.413 | *51.658* |
| **[roxmltree](https://crates.io/crates/roxmltree)** | 0.721 | 262.573 | 2295.695 | 3730.335 |
| **[xmltree](https://crates.io/crates/xmltree)** | 3.754 | 2466.145 | 4103.327 | - |
| **[treexml](https://crates.io/crates/treexml)** | 4.08 | 2304.143 | 4203.388 | 18200.684 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    dummy_xml = "0.1.6"          # Fast Non-validating XML DOM parser.
    quick-xml = "0.13.1"          # High performance xml reader and writer
    roxmltree = "0.4.0"    # Represent an XML as a read-only tree.
    xmltree = "0.8.0"      # Parse an XML file into a simple tree-like structure
    treexml = "0.7.0"      # An XML tree library for Rust

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`