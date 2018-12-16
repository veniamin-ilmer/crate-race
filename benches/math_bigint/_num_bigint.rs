use bencher::Bencher;

use num_bigint::BigUint;
use num_traits::One;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let num_1: BigUint = One::one();
    let mut num_sum: BigUint = One::one();
    num_sum += &num_1;
    assert_eq!(num_sum.to_string(), super::BASELINE);
  });
}


pub fn fact95(b: &mut Bencher) {
  b.iter(|| {
    let num_1: BigUint = One::one();
    let mut num_sum: BigUint = One::one();
    let mut fact: BigUint = One::one();
    for _ in 1..95 {
      num_sum += &num_1;
      fact *= &num_sum;
    }
    assert_eq!(fact.to_string(), super::FACT95);
  });
}