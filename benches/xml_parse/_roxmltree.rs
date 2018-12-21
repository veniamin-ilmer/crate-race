use bencher::Bencher;

use roxmltree::Document;

//https://docs.rs/roxmltree/0.4.0/roxmltree/struct.Node.html#method.text
pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = "test";
    let document = Document::parse(super::BASELINE).unwrap();
    let root = document.root_element();
    let result = root.text();
    assert_eq!(result, Some(expected_answer));
  });
}

//https://docs.rs/roxmltree/0.4.0/roxmltree/struct.Node.html#method.attribute
pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = "test1000";
    let document = Document::parse(super::ATTRIBUTE).unwrap();
    let root = document.root_element();
    let result = root.attribute("att1000");
    assert_eq!(result, Some(expected_answer));
  });
}


pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = "test";
    let document = Document::parse(super::SERIAL).unwrap();
    let tag = document.root_element().last_child().unwrap();
    let result = tag.text();
    assert_eq!(result, Some(expected_answer));
  });
}


pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = "test";

    let document = Document::parse(super::NESTED).unwrap();
    let mut tag = document.root_element();
    for _ in 0..1000 {
      tag = tag.first_child().unwrap();
    }
    let result = tag.text();
    assert_eq!(result, Some(expected_answer));
  });
}
