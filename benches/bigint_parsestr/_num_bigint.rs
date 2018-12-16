use bencher::Bencher;

use num_bigint::BigUint;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let num: BigUint = super::BASELINE.parse::<BigUint>().unwrap();
    assert_eq!(num.to_string(), super::BASELINE);
  });
}


pub fn big_num(b: &mut Bencher) {
  b.iter(|| {
    let num: BigUint = super::BIG_NUM.parse::<BigUint>().unwrap();
    assert_eq!(num.to_string(), super::BIG_NUM);
  });
}
