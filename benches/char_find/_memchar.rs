use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(memchr::memchr(b'/', super:BASELINE), Some(0));
  });
}

pub fn big_str(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(memchr::memchr(b'/', super:BIG_STR), "/");
  });
}
