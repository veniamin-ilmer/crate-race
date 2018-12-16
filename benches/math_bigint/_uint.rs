use bencher::Bencher;

use uint::U512;

pub fn baseline(b: &mut Bencher) {
  
  b.iter(|| {
    let num_1 = U512::from(1);
    let mut num_sum = U512::from(1);
    num_sum += num_1;
    assert_eq!(num_sum.to_string(), super::BASELINE);
  });
}


pub fn fact95(b: &mut Bencher) {
  b.iter(|| {
    let num_1 = U512::from(1);
    let mut num_sum = U512::from(1);
    let mut fact = U512::from(1);
    for _ in 1..95 {
      num_sum += num_1;
      fact *= num_sum;
    }
    assert_eq!(fact.to_string(), super::FACT95);
  });
}