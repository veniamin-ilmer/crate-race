use bencher::Bencher;

use dummy_xml::parser;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = parser::parse_str(super::BASELINE).unwrap();
    let tag = document.root().first_child().unwrap();
    let result = tag.value();
    assert_eq!(result, expected_answer);
  });
}

pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test1000");

    let document = parser::parse_str(super::ATTRIBUTE).unwrap();
    let tag = document.root();
    let result = tag.last_attribute().unwrap().value();
    assert_eq!(result, expected_answer);
  });
}

/* fails to parse when there are multiple root tags
pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");

    let document = parser::parse_str(super::SERIAL).unwrap();
    let tag = document.root().last_child().unwrap();
    let result = tag.value();
    assert_eq!(result, expected_answer);
  });
}
*/

pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");

    let document = parser::parse_str(super::NESTED).unwrap();
    let mut tag = document.root();
    for _ in 0..1000 {
      tag = tag.first_child().unwrap();
    }
    let result = tag.value();
    assert_eq!(result, expected_answer);
  });
}
