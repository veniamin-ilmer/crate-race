# random_shuffle
Randomly shuffling arrays.

* **Baseline**: Array with length of 1.
* **Thousand**: Array with length of 1,000.
* **Million**: Array with length of 1,000,000.

The `rand` crate uses the `rand_hc` algorithm.

| | baseline | thousand | million |
| --- | --- | --- | --- |
| **[rand_pcg::Mcg128Xsl64](https://crates.io/crates/rand_pcg)** | 0 | *8.28* | *9061.68* |
| **[rand_xorshift](https://crates.io/crates/rand_xorshift)** | 0.002 | 8.304 | 9116.448 |
| **[rand_hc](https://crates.io/crates/rand_hc)** | 0.003 | 10.25 | 11562.716 |
| **[rand_pcg::Lcg64Xsh32](https://crates.io/crates/rand_pcg)** | *0* | 10.783 | 11685.453 |
| **[rand_isaac](https://crates.io/crates/rand_isaac)** | 0.003 | 12.998 | 13788.521 |
| **[rand_chacha](https://crates.io/crates/rand_chacha)** | 0.003 | 26.367 | 27263.428 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    rand_pcg = "0.1.1"      # Selected PCG random number generators 
    rand_xorshift = "0.1.1"    # Xorshift random number generator 
    rand_hc = "0.1.0"          # HC128 random number generator 
    rand_pcg = "0.1.1"      # Selected PCG random number generators 
    rand_isaac = "0.1.1"    # ISAAC random number generator 
    rand_chacha = "0.1.1"    # ChaCha random number generator 

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`