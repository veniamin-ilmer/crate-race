use bencher::Bencher;

use rand::prelude::*;
use rand_pcg::Lcg64Xsh32;
use rand::SeedableRng;

pub fn baseline(b: &mut Bencher) {
  let mut arr: Vec<u32> = (0..1).collect();
  let mut rng: Lcg64Xsh32 = SeedableRng::from_seed([0; 16]);
  b.iter(|| {
    arr.shuffle(&mut rng);
    assert_eq!(arr.len(), 1);
  });
}

pub fn thousand(b: &mut Bencher) {
  let mut arr: Vec<u32> = (0..1_000).collect();
  let mut rng: Lcg64Xsh32 = SeedableRng::from_seed([0; 16]);
  b.iter(|| {
    arr.shuffle(&mut rng);
    assert_eq!(arr.len(), 1_000);
  });
}

pub fn million(b: &mut Bencher) {
  let mut arr: Vec<u32> = (0..1_000_000).collect();
  let mut rng: Lcg64Xsh32 = SeedableRng::from_seed([0; 16]);
  b.iter(|| {
    arr.shuffle(&mut rng);
    assert_eq!(arr.len(), 1_000_000);
  });
}
