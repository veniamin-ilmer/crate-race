//!Randomly shuffling arrays.
//!
//!* **Baseline**: Array with length of 1.
//!* **Thousand**: Array with length of 1,000.
//!* **Million**: Array with length of 1,000,000.
//!
//!The `rand` crate uses the `rand_hc` algorithm.

#[macro_use]
extern crate bencher;

mod _rand_chacha;
mod _rand_hc;
mod _rand_isaac;
mod _rand_pcg;
mod _rand_xorshift;

benchmark_group!(baseline, _rand_chacha::baseline, _rand_hc::baseline, _rand_isaac::baseline, _rand_pcg::Lcg64Xsh32::baseline, _rand_pcg::Mcg128Xsl64::baseline, _rand_xorshift::baseline);
benchmark_group!(thousand, _rand_chacha::thousand, _rand_hc::thousand, _rand_isaac::thousand, _rand_pcg::Lcg64Xsh32::thousand, _rand_pcg::Mcg128Xsl64::thousand, _rand_xorshift::thousand);
benchmark_group!(million, _rand_chacha::million, _rand_hc::million, _rand_isaac::million, _rand_pcg::Lcg64Xsh32::million, _rand_pcg::Mcg128Xsl64::million, _rand_xorshift::million);

benchmark_main!(baseline, thousand, million);
