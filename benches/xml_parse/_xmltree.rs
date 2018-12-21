use bencher::Bencher;

use xmltree::Element;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Element::parse(super::BASELINE.as_bytes()).unwrap();
    let result = document.text;
    assert_eq!(result, Some(expected_answer));
  });
}

pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test1000");
    let document = Element::parse(super::ATTRIBUTE.as_bytes()).unwrap();
    let attr = document.attributes;
    let result = attr.get("att1000").unwrap().clone();
    assert_eq!(result, expected_answer);
  });
}

pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Element::parse(super::SERIAL.as_bytes()).unwrap();
    let tag = document.get_child("tag1000").unwrap();
    let result = tag.text.clone();
    assert_eq!(result, Some(expected_answer));
  });
}

/*Stack overflowed with this code
pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Element::parse(super::NESTED.as_bytes()).unwrap();
    let mut tag = &document;
    for _ in 0..1000 {
      tag = &tag.children[0];  //Get the first (and only) child each time
    }
    let result = tag.text.clone();
    assert_eq!(result, Some(expected_answer));
  });
}*/