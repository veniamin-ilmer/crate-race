use bencher::Bencher;

use quick_xml::Reader;
use quick_xml::events::Event;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let mut reader = Reader::from_str(super::BASELINE);

    let mut result = String::new();
    let expected_answer = String::from("test");
    
    let mut buf = Vec::new();

    let mut tagged = false;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"tag" {
                    tagged = true;
                }
            }
            Ok(Event::Text(e)) => {
                if tagged {
                    result = e.unescape_and_decode(&reader).unwrap();
                    break;
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    
    assert_eq!(result, expected_answer);
    
  });
}


pub fn attribute(b: &mut Bencher) {
  b.iter(|| {
    let mut reader = Reader::from_str(super::ATTRIBUTE);

    let mut result = String::new();
    let expected_answer = String::from("test1000");
    
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"tag" {
                    //This was not fun.. Let me know if there is an easier way to do this.
                    result = String::from_utf8(e.attributes().nth(999).unwrap().unwrap().value.into_owned()).unwrap();
                    break;
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    
    assert_eq!(result, expected_answer);
    
  });
}


pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let mut reader = Reader::from_str(super::SERIAL);

    let mut result = String::new();
    let expected_answer = String::from("test");
    
    let mut buf = Vec::new();

    let mut tagged = false;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"tag1000" {
                    tagged = true;
                }
            }
            Ok(Event::Text(e)) => {
                if tagged {
                    result = e.unescape_and_decode(&reader).unwrap();
                    break;
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    
    assert_eq!(result, expected_answer);
    
  });
}


pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let mut reader = Reader::from_str(super::NESTED);

    let mut result = String::new();
    let expected_answer = String::from("test");
    
    let mut buf = Vec::new();

    let mut tagged = false;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"tag1000" {
                    tagged = true;
                }
            }
            Ok(Event::Text(e)) => {
                if tagged {
                    result = e.unescape_and_decode(&reader).unwrap();
                    break;
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    
    assert_eq!(result, expected_answer);
    
  });
}