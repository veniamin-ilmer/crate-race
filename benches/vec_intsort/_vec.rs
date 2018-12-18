use bencher::Bencher;

use rand::{thread_rng, Rng};

pub fn baseline_small(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10).collect();
    vec.sort();
    assert_eq!(test_vec, answer_vec);
    thread_rng().shuffle(&mut vec); //This extra useless shuffle command is here to try and normalize it with the random benchmark.
  });
}

pub fn random_small(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10).collect();
    thread_rng().shuffle(&mut vec);
    vec.sort();
    assert_eq!(test_vec, answer_vec);
  });
}

pub fn reverse_small(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10).rev().collect();
    vec.sort();
    assert_eq!(test_vec, answer_vec);
    thread_rng().shuffle(&mut vec); //This extra useless shuffle command is here to try and normalize it with the random benchmark.
  });
}

pub fn baseline_big(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10_000).collect();
    vec.sort();
    assert_eq!(test_vec, answer_vec);
    thread_rng().shuffle(&mut vec); //This extra useless shuffle command is here to try and normalize it with the random benchmark.
  });
}

pub fn random_big(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10_000).collect();
    thread_rng().shuffle(&mut vec);
    vec.sort();
    assert_eq!(test_vec, answer_vec);
  });
}

pub fn reverse_big(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10_000).rev().collect();
    vec.sort();
    assert_eq!(test_vec, answer_vec);
    thread_rng().shuffle(&mut vec); //This extra useless shuffle command is here to try and normalize it with the random benchmark.
  });
}
