# random_shuffle
Randomly shuffling arrays.

* **Baseline**: Array with length of 1.
* **Thousand**: Array with length of 1,000.
* **Million**: Array with length of 1,000,000.

| | baseline | thousand | million |
| --- | --- | --- | --- |
| **[rand_pcg::Mcg128Xsl64](https://crates.io/crates/rand_pcg)** | *0* | *8.119* | *10139.089* |
| **[rand_hc](https://crates.io/crates/rand_hc)** | 0.003 | 9.76 | 10869.213 |
| **[rand_xorshift](https://crates.io/crates/rand_xorshift)** | 0.002 | 8.173 | 11395.009 |
| **[rand_pcg::Lcg64Xsh32](https://crates.io/crates/rand_pcg)** | 0 | 10.385 | 13407.178 |
| **[rand_isaac](https://crates.io/crates/rand_isaac)** | 0.003 | 13.011 | 14767.657 |
| **[rand_chacha](https://crates.io/crates/rand_chacha)** | 0.003 | 25.276 | 27718.848 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    rand_pcg = "0.1.1"      # Selected PCG random number generators 
    rand_hc = "0.1.0"          # HC128 random number generator 
    rand_xorshift = "0.1.0"    # Xorshift random number generator 
    rand_pcg = "0.1.1"      # Selected PCG random number generators 
    rand_isaac = "0.1.1"    # ISAAC random number generator 
    rand_chacha = "0.1.0"    # ChaCha random number generator 

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`