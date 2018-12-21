use bencher::Bencher;

use treexml::Document;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::BASELINE.as_bytes()).unwrap();
    let root = document.root.unwrap();
    let result = root.text.clone();
    assert_eq!(result, Some(expected_answer));
  });
}

pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = "test1000";
    let document = Document::parse(super::ATTRIBUTE.as_bytes()).expect("1");
    let root = document.root.expect("2");
    let result = root.attributes.get("att1000").expect("4");
    assert_eq!(result, expected_answer);
  });
}

pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::SERIAL.as_bytes()).unwrap();
    let root = document.root.unwrap();
    let tag = root.find_child(|tag| tag.name == "tag1000").unwrap();
    let result = &tag.text;
    assert_eq!(result, &Some(expected_answer));
  });
}

pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let expected_answer = String::from("test");
    let document = Document::parse(super::NESTED.as_bytes()).unwrap();
    let mut tag = &document.root.unwrap();
    for _ in 0..999 {
      tag = &tag.children[0];  //Get the first (and only) child each time
    }
    let result = &tag.text;
    assert_eq!(result, &Some(expected_answer));
  });
}
