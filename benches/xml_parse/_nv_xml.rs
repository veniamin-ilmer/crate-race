use bencher::Bencher;

use nv_xml::XmlParser;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = XmlParser::parse(super::BASELINE).unwrap();
    let result = document.data().unwrap();
    assert_eq!(result, expected_answer);
  });
}

pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test1000");
    let document = XmlParser::parse(super::ATTRIBUTE).unwrap();
    result = document.attribute_value_of("attr1000");
    assert_eq!(result, expected_answer);
  });
}

pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = XmlParser::parse(super::SERIAL).unwrap();
    let tag = document.children()[0];
    let result = tag.data().unwrap();
    assert_eq!(result, expected_answer);
  });
}

pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let tag = XmlParser::parse(super::NESTED).unwrap();
    for _ in 0..1000 {
      let tag = tag.children()[0];  //Get the first (and only) child each time
    }
    let result = tag.data().unwrap();
    assert_eq!(result, expected_answer);
  });
}
