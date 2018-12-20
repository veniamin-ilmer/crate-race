# xml_parse
Parsing XML, and retrieving specific text or attribute.

* **Baseline**: Simple XML with just one tag, no attributes.
* **Attributes**: 1 tag with 1000 attributes.
* **Serial**: 1000 tags, opening and closing, opening and closing. No nesting.
* **Nested**: 1000 nested entries, each inside of the other.

| | baseline | serial | nested | attribute |
| --- | --- | --- | --- | --- |
| **[dummy_xml](https://crates.io/crates/dummy_xml)** | *0.239* | - | 143.274 | *194.841* |
| **[quick_xml](https://crates.io/crates/quick_xml)** | 0.324 | *90.701* | *51.064* | 2932.532 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    dummy_xml = "0.1.6"          # Fast Non-validating XML DOM parser.
    quick-xml = "0.13.1"          # High performance xml reader and writer

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`