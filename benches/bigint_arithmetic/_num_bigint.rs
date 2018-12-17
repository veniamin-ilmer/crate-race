use bencher::Bencher;

use num_bigint::BigUint;
use num_traits::One;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let num_1: BigUint = One::one();
    let mut num_sum: BigUint = One::one();
    num_sum += &num_1;
    assert_eq!(BigUint::from(2u8), num_sum);
  });
}


pub fn fact50(b: &mut Bencher) {
  b.iter(|| {
    let num_1: BigUint = One::one();
    let mut num_sum: BigUint = One::one();
    let mut fact: BigUint = One::one();
    for _ in 1..50 {
      num_sum += &num_1;
      fact *= &num_sum;
    }
    
    let ten = BigUint::from(10u8);
    assert_eq!(BigUint::from(0u8), &fact % &ten); //Most right digit
    for _ in 0..64 {    //Remove all other digits
      fact /= &ten;
    }
    assert_eq!(BigUint::from(3u8), fact); //Most left digit
  });
}


pub fn fact95(b: &mut Bencher) {
  b.iter(|| {
    let num_1: BigUint = One::one();
    let mut num_sum: BigUint = One::one();
    let mut fact: BigUint = One::one();
    for _ in 1..95 {
      num_sum += &num_1;
      fact *= &num_sum;
    }
    
    let ten = BigUint::from(10u8);
    assert_eq!(BigUint::from(0u8), &fact % &ten); //Most right digit
    for _ in 0..148 {    //Remove all other digits
      fact /= &ten;
    }
    assert_eq!(BigUint::from(1u8), fact); //Most left digit
  });
}
