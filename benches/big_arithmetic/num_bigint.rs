fn factorial() -> BigUint {
    let n1: BigUint = One::one();
    let mut ns: BigUint = One::one();
    let mut factn: BigUint = One::one();
    for _ in 1..95 {
        ns += &n1;
        factn *= &ns;
    }
    factn
}

fn main() {
  assert_eq!(factorial().to_string(), super::fact95);
}
