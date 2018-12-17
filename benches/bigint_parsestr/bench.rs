//!Parsing large numbers from strings.
//!
//!* **Baseline**: Convert "1" string into a number, and then back into a string again.
//!* **Big_num**: Convert a 149 digit string into a number, and then back into a string again.

#[macro_use]
extern crate bencher;

static BASELINE: &str = "1";
static BIG_NUM: &str = "10329978488239059262599702099394727095397746340117372869212250571234293987594703124871765375385424468563282236864226607350415360000000000000000000000";

mod _num_bigint;
mod _uint;
mod _numext_fixed_uint;

benchmark_group!(baseline, _num_bigint::baseline, _uint::baseline, _numext_fixed_uint::baseline);
benchmark_group!(big_num, _num_bigint::big_num, _uint::big_num, _numext_fixed_uint::big_num);

benchmark_main!(baseline, big_num);
