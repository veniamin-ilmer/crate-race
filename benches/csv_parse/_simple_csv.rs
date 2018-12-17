use bencher::Bencher;
use simple_csv::SimpleCsvReader;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = SimpleCsvReader::new(super::BASELINE.as_bytes());
    //There is no "has header" option, so we are going to manually skip it.

    let _ = rdr.next(); //header
    let row = rdr.next().unwrap().unwrap();
    assert_eq!(row[0], "Data");
  });
}

pub fn rows(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = SimpleCsvReader::new(super::ROWS.as_bytes());

    let _ = rdr.next(); //header
    let row = rdr.nth(999).unwrap().unwrap();  //999 is the 1000th row
    assert_eq!(row[0], "Row1000");
  });
}

pub fn headers(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = SimpleCsvReader::new(super::HEADERS.as_bytes());

    let _ = rdr.next(); //header
    let row = rdr.next().unwrap().unwrap();
    assert_eq!(row[999], "Data1000");  //999 is the 1000th column
  });
}
