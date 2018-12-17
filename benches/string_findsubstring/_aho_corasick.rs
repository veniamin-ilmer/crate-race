use bencher::Bencher;

use aho_corasick::{Automaton, AcAutomaton, Match};

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let aut = AcAutomaton::new(vec![super::BASELINE]);
    let mut it = aut.find(super::BASELINE1);
    assert_eq!(it.next(), Some(Match { pati: 0, start: 1, end: 7}));
  });
}

pub fn big_pattern(b: &mut Bencher) {
  b.iter(|| {
    let aut = AcAutomaton::new(vec![super::BIG_PATTERN]);
    let mut it = aut.find(super::BIG_SEARCH);
    assert_eq!(it.next(), Some(Match { pati: 0, start: 1, end: 50_001}));
  });
}

pub fn almost(b: &mut Bencher) {
  b.iter(|| {
    let aut = AcAutomaton::new(vec![super::BASELINE]);
    let mut it = aut.find(super::ALMOST);
    assert_eq!(it.next(), Some(Match { pati: 0, start: 50_000, end: 50_006}));
  });
}

pub fn monotonous(b: &mut Bencher) {
  b.iter(|| {
    let aut = AcAutomaton::new(vec![super::BASELINE]);
    let mut it = aut.find(super::MONOTONOUS);
    assert_eq!(it.next(), Some(Match { pati: 0, start: 50_000, end: 50_006}));
  });
}
