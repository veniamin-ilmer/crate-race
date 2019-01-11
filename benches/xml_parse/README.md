# xml_parse
Parsing XML, and retrieving specific text or attribute.

* **Baseline**: Simple XML with just one tag, no attributes.
* **Attributes**: 1 tag with 1000 attributes.
* **Serial**: 1000 tags, opening and closing, opening and closing. No nesting.
* **Nested**: 1000 nested entries, each inside of the other.

| | baseline | serial | attribute | nested |
| --- | --- | --- | --- | --- |
| **[quick_xml](https://crates.io/crates/quick_xml)** | 0.331 | *95.667* | *36.364* | *54.551* |
| **[dummy_xml](https://crates.io/crates/dummy_xml)** | *0.257* | 144.144 | 200.536 | 145.042 |
| **[roxmltree](https://crates.io/crates/roxmltree)** | 0.761 | 281.645 | 2667.008 | 3906.095 |
| **[xmltree](https://crates.io/crates/xmltree)** | 3.895 | 2567.188 | 4547.001 | - |
| **[treexml](https://crates.io/crates/treexml)** | 4.215 | 2541.496 | 4419.271 | 18896.049 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    quick-xml = "0.13.1"         # High performance xml reader and writer
    dummy_xml = "0.1.6"          # Fast Non-validating XML DOM parser.
    roxmltree = "0.4.1"    # Represent an XML as a read-only tree.
    xmltree = "0.8.0"      # Parse an XML file into a simple tree-like structure
    treexml = "0.7.0"      # An XML tree library for Rust

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`