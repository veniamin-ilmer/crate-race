///Baseline: Add 1 + 1
///Fact95: Take the factorial of 95.

#[macro_use]
extern crate bencher;

static BASELINE: &str = "2";
static FACT95: &str = "10329978488239059262599702099394727095397746340117372869212250571234293987594703124871765375385424468563282236864226607350415360000000000000000000000";

mod _num_bigint;
mod _uint;
mod _numext_fixed_uint;

benchmark_group!(baseline, _num_bigint::baseline, _uint::baseline, _numext_fixed_uint::baseline);
benchmark_group!(fact95, _num_bigint::fact95, _uint::fact95, _numext_fixed_uint::fact95);

benchmark_main!(baseline, fact95);
