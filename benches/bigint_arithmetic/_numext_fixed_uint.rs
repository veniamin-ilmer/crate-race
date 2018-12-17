use bencher::Bencher;

use numext_fixed_uint::{U256, U512};

pub fn baseline(b: &mut Bencher) {
  
  b.iter(|| {
    let num_1 = U512::one();
    let mut num_sum = U512::one();
    num_sum *= &num_1;
    assert_eq!(num_sum.to_string(), super::BASELINE);
  });
}


pub fn fact50(b: &mut Bencher) {
  b.iter(|| {
    let num_1 = U256::one();
    let mut num_sum = U256::one();
    let mut fact = U256::one();
    for _ in 1..50 {
      num_sum += &num_1;
      fact *= &num_sum;
    }
    assert_eq!(fact.to_string(), super::FACT50);
  });
}


pub fn fact95(b: &mut Bencher) {
  b.iter(|| {
    let num_1 = U512::one();
    let mut num_sum = U512::one();
    let mut fact = U512::one();
    for _ in 1..95 {
      num_sum += &num_1;
      fact *= &num_sum;
    }
    assert_eq!(fact.to_string(), super::FACT95);
  });
}
