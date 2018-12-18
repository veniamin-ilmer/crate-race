use bencher::Bencher;

use algos::pattern;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::bruteforce(super::BASELINE1, super::BASELINE), Some(1));
  });
}

pub fn big_pattern(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::bruteforce(super::BIG_SEARCH, super::BIG_PATTERN), Some(1));
  });
}

pub fn almost(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::bruteforce(super::ALMOST, super::BASELINE), Some(50_000));
  });
}

pub fn monotonous(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::bruteforce(super::MONOTONOUS, super::BASELINE), Some(50_000));
  });
}
