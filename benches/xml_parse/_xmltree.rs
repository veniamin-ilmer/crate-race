use bencher::Bencher;

use xmltree::Element;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Element::parse(super::BASELINE.as_bytes()).unwrap();
    let result = document.text;
    assert_eq!(result, expected_answer);
  });
}

pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test1000");
    let document = Element::parse(super::ATTRIBUTE).unwrap();
    let attr = document.attributes;
    let result = *attr.get("attr1000").unwrap();
    assert_eq!(result, expected_answer);
  });
}

pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Element::parse(super::SERIAL).unwrap();
    let tag = document.get_child("tag1000").unwrap();
    let result = tag.test;
    assert_eq!(result, expected_answer);
  });
}


pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Element::parse(super::NESTED).unwrap();
    //TODO - finish this code
    assert_eq!(result, expected_answer);
  });
}
