use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(subslice::bmh::find(super::BASELINE1, super::BASELINE), Some(1));
  });
}

pub fn big_pattern(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(subslice::bmh::find(super::BIG_SEARCH, super::BIG_PATTERN), Some(1));
  });
}

pub fn almost(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(subslice::bmh::find(super::ALMOST, super::BASELINE), Some(50_000));
  });
}

pub fn monotonous(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(subslice::bmh::find(super::MONOTONOUS, super::BASELINE), Some(50_000));
  });
}
