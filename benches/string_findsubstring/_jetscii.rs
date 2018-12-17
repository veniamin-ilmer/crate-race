use bencher::Bencher;

use jetscii::ByteSubstring;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(ByteSubstring::new(super::BASELINE).find(super::BASELINE1), Some(1));
  });
}

pub fn big_pattern(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(ByteSubstring::new(super::BIG_PATTERN).find(super::BIG_SEARCH), Some(1));
  });
}

pub fn almost(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(ByteSubstring::new(super::BASELINE).find(super::ALMOST), Some(50_000));
  });
}

pub fn monotonous(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(ByteSubstring::new(super::BASELINE).find(super::MONOTONOUS), Some(50_000));
  });
}
