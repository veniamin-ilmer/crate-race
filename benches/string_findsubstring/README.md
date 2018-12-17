# string_findsubstring
Search for the position of a string inside of a string.

* Baseline: Searching "abc123" inside of "1abc123"
* Big_Pattern: Search for a 50kb string inside of "1" + the 50kb string. Chars for this string were random.
* Monotonous: 50kb string of all 1s. Find "abc123" at the end.
* Almost: 50kb string repeating "abc12" without the ending "3". Find "abc123" at the end of the string.

| | baseline | monotonous | almost | big_pattern |
| --- | --- | --- | --- | --- |
| **[jetscii](https://crates.io/crates/jetscii)** | 0.019 | 7.947 | 38.077 | *2.24* |
| **[subslice](https://crates.io/crates/subslice)** | 0.032 | *0.526* | 30.882 | 28.045 |
| **[twoway](https://crates.io/crates/twoway)** | 0.024 | 4.96 | *27.326* | 100.316 |
| **[galil_seiferas](https://crates.io/crates/galil_seiferas)** | *0.017* | 59.138 | 59.58 | 114.507 |
| **[aho_corasick](https://crates.io/crates/aho_corasick)** | 2.067 | 2.734 | 247.246 | 6090.251 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    jetscii = "0.4.3"    # A tiny library to efficiently search strings and byte slices for sets of ASCII characters or bytes.
    subslice = "0.2.0"          # Fast subslice search
    twoway = "0.2.0"        # Fast substring search for strings and byte strings. Optional SSE4.2 acceleration (if detected at runtim…
    galil-seiferas = "0.1.5"    # General string search in constant space, linear time, for nonorderable alphabets.
    aho-corasick = "0.6.9"      # Fast multiple substring searching with finite state machines.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`