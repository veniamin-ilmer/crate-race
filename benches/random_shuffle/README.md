# random_shuffle
Randomly shuffling arrays.

* **Baseline**: Array with length of 1.
* **Thousand**: Array with length of 1,000.
* **Million**: Array with length of 1,000,000.

The `rand` crate uses the `rand_hc` algorithm.

| | baseline | thousand | million |
| --- | --- | --- | --- |
| **[rand_xorshift](https://crates.io/crates/rand_xorshift)** | 0.002 | *7.694* | *8503.846* |
| **[rand_pcg::Mcg128Xsl64](https://crates.io/crates/rand_pcg)** | 0 | 7.696 | 8596.41 |
| **[rand_pcg::Lcg64Xsh32](https://crates.io/crates/rand_pcg)** | *0* | 9.033 | 10409.24 |
| **[rand_hc](https://crates.io/crates/rand_hc)** | 0.003 | 9.693 | 10716.552 |
| **[rand_isaac](https://crates.io/crates/rand_isaac)** | 0.003 | 12.138 | 12809.92 |
| **[rand_chacha](https://crates.io/crates/rand_chacha)** | 0.003 | 24.542 | 25337.973 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    rand_xorshift = "0.1.0"    # Xorshift random number generator 
    rand_pcg = "0.1.1"      # Selected PCG random number generators 
    rand_pcg = "0.1.1"      # Selected PCG random number generators 
    rand_hc = "0.1.0"          # HC128 random number generator 
    rand_isaac = "0.1.1"    # ISAAC random number generator 
    rand_chacha = "0.1.0"    # ChaCha random number generator 

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`