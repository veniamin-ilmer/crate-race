use bencher::Bencher;

use needle::boyer_moore::BoyerMoore;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(BoyerMoore::new(BASELINE).find_first_in(super::BASELINE1), Some(1));
  });
}

pub fn big_pattern(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(BoyerMoore::new(BIG_PATTERN).find_first_in(super::BIG_SEARCH), Some(1));
  });
}

pub fn almost(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(BoyerMoore::new(BASELINE).find_first_in(super::ALMOST), Some(50_000));
  });
}

pub fn monotonous(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(BoyerMoore::new(BASELINE).find_first_in(super::MONOTONOUS), Some(50_000));
  });
}
