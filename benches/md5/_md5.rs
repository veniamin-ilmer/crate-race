use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let digest = md5::compute(super::BASELINE_INPUT);
    assert_eq!(format!("{:x}", digest), super::BASELINE_OUTPUT);
  });
}

pub fn lorem(b: &mut Bencher) {
  b.iter(|| {
    let digest = md5::compute(super::LOREM_INPUT);
    assert_eq!(format!("{:x}", digest), super::LOREM_OUTPUT);
  });
}