use bencher::Bencher;

use md_5::{Md5, Digest};

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let digest = Md5::digest(super::BASELINE_INPUT);
    assert_eq!(format!("{:x}", digest), super::BASELINE_OUTPUT);
  });
}

pub fn lorem(b: &mut Bencher) {
  b.iter(|| {
    let digest = Md5::digest(super::LOREM_INPUT);
    assert_eq!(format!("{:x}", digest), super::LOREM_OUTPUT);
  });
}
