use bencher::Bencher;

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
    let mut rdr = SimpleCsvReader::new(super::ROWS.as_bytes())

    let _ = rdr.next(); //header
    let row = rdr.nth(99).unwrap().unwrap();  //99 is the 100th row
    assert_eq!(row[0].unwrap(), "Row100");
  });
}

pub fn headers(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = SimpleCsvReader::new(super::HEADERS.as_bytes())

    let _ = rdr.next(); //header
    let row = rdr.next().unwrap().unwrap();
    assert_eq!(row[99], "Data100");  //99 is the 100th column
  });
}
