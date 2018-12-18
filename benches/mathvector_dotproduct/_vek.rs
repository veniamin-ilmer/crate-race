use bencher::Bencher;

use vek::Vec3;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let v1 = Vec3::new(2.0,3.0,4.0);
    let v2 = Vec3::new(5.0,6.0,7.0);
    assert_eq!(56.0, v1.dot(v2));
  });
}
