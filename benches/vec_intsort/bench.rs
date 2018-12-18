//!Sorting Vecs of u32.
//!
//!* *Baseline_small*: 10 items. Fully sorted already.
//!* *Random_small = 10 items. Completely random.
//!* *Reverse_small = 10 items. Sorted in reverse order.
//!* *Baseline_big = 10,000 items. Fully sorted already.
//!* *Random_big = 10,000 items. Completely random.
//!* *Reverse_big = 10,000 items. Sorted in reverse order.
//!
//!"vec" is the out of the box `sort` function that comes with Vec.

#[macro_use]
extern crate bencher;

benchmark_group!(baseline_small, _vec::baseline_small);
benchmark_group!(random_small, _vec::random_small);
benchmark_group!(reverse_small, _vec::reverse_small);
benchmark_group!(baseline_big, _vec::baseline_big);
benchmark_group!(random_big, _vec::random_big);
benchmark_group!(reverse_big, _vec::reverse_big);

benchmark_main!(baseline_small, random_small, reverse_small, baseline_big, random_big, reverse_big);
