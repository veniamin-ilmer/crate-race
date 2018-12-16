use bencher::Bencher;

use numext_fixed_uint::U512;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let num = U512::from_dec_str(super::BASELINE).unwrap();
    assert_eq!(num.to_string(), super::BASELINE);
  });
}


pub fn big_num(b: &mut Bencher) {
  b.iter(|| {
    let num = U512::from_dec_str(super::BIG_NUM).unwrap();
    assert_eq!(num.to_string(), super::BIG_NUM);
  });
}