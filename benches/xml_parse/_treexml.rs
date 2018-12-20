use bencher::Bencher;

use treexml::Document;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::BASELINE.as_bytes()).unwrap();
    let root = doc.root.unwrap();
    let tag = root.find_child(|tag| tag.name == "tag").unwrap();
    let result = tag.contents.unwrap();
    assert_eq!(result, expected_answer);
  });
}

pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test1000");
    let document = Document::parse(super::ATTRIBUTE).unwrap();
    let root = doc.root.unwrap();
    let tag = root.find_child(|tag| tag.name == "tag").unwrap();
    let result = tag.attributes.get("attr1000").unwrap();
    assert_eq!(result, expected_answer);
  });
}

pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::SERIAL).unwrap();
    let root = doc.root.unwrap();
    let tag = root.find_child(|tag| tag.name == "tag1000").unwrap();
    let result = tag.contents.unwrap();
    assert_eq!(result, expected_answer);
  });
}

pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::NESTED).unwrap();
    let tag = doc.root.unwrap();
    for _ in 0..1000 {
      tag = tag.children[0];  //Get the first (and only) child each time
    }
    let result = tag.contents.unwrap();
    assert_eq!(result, expected_answer);
  });
}
