use bencher::Bencher;

use cgmath::Vector3;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let v1 = Vector3::new(2.0,3.0,4.0);
    let v2 = Vector3::new(5.0,6.0,7.0);
    assert_eq!(56.0, cgmath::dot(v1, v2));
  });
}

//cgmath doesn't allow to do more than 4 dimensions