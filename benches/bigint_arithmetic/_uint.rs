use bencher::Bencher;

use uint::{U256, U512};

pub fn baseline(b: &mut Bencher) {
  
  b.iter(|| {
    let num_1 = U512::from(1);
    let mut num_sum = U512::from(1);
    num_sum += num_1;
    assert_eq!(U512::from(2), num_sum);
  });
}


pub fn fact50(b: &mut Bencher) {
  b.iter(|| {
    let num_1 = U256::from(1);
    let mut num_sum = U256::from(1);
    let mut fact = U256::from(1);
    for _ in 1..50 {
      num_sum += num_1;
      fact *= num_sum;
    }

    let ten = U256::from(10);
    assert_eq!(U256::from(0), fact % ten); //Most right digit
    for _ in 0..64 {    //Remove all other digits
      fact /= ten;
    }
    assert_eq!(U256::from(3), fact); //Most left digit
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
    
    let ten = U256::from(10);
    assert_eq!(U256::from(0), fact % ten); //Most right digit
    for _ in 0..148 {    //Remove all other digits
      fact /= ten;
    }
    assert_eq!(U256::from(1), fact); //Most left digit
  });
}
