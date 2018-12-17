use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(twoway::find_bytes(super::BASELINE1, super::BASELINE), Some(1));
  });
}

pub fn big_pattern(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(twoway::find_bytes(super::BIG_SEARCH, super::BIG_PATTERN), Some(1));
  });
}

pub fn almost(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(twoway::find_bytes(super::ALMOST, super::BASELINE), Some(50_000));
  });
}

pub fn monotonous(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(twoway::find_bytes(super::MONOTONOUS, super::BASELINE), Some(50_000));
  });
}
