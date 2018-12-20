use bencher::Bencher;

let doc = ::parse("<e/>").unwrap();

assert!(doc.root().is_root());
assert!(doc.root().first_child().unwrap().has_tag_name("e"));
use roxmltree::Document;

//https://docs.rs/roxmltree/0.4.0/roxmltree/struct.Node.html#method.text
pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::BASELINE).unwrap();
    let result = document.root_element().text();
    assert_eq!(result, Some(expected_answer));
  });
}

https://docs.rs/roxmltree/0.4.0/roxmltree/struct.Node.html#method.attribute
pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test1000");
    let document = Document::parse(super::ATTRIBUTE).unwrap();
    let result = document.root_element().attribute("attr1000");
    assert_eq!(result, Some(expected_answer));
  });
}


pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::SERIAL).unwrap();
    let tag = document.root_element().last_child().unwrap();
    let result = tag.text();
    assert_eq!(result, Some(expected_answer));
  });
}


pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");

    let document = Document::parse(super::NESTED).unwrap();
    let mut tag = document.root_element();
    for _ in 0..1000 {
      tag = tag.first_child().unwrap();
    }
    let result = tag.text();
    assert_eq!(result, Some(expected_answer));
  });
}
