use bencher::Bencher;

use csv::ReaderBuilder;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = ReaderBuilder::new()
       .from_reader(super::BASELINE.as_bytes());

    let row = rdr.records().next().unwrap().unwrap();
    assert_eq!(&row[0], "Data");
  });
}

pub fn rows(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = ReaderBuilder::new()
       .from_reader(super::ROWS.as_bytes());

    let row = rdr.records().nth(99).unwrap().unwrap();  //99 is the 100th row
    assert_eq!(&row[0], "Row100");
  });
}

pub fn headers(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = ReaderBuilder::new()
       .from_reader(super::HEADERS.as_bytes());

    let row = rdr.records().next().unwrap().unwrap();
    assert_eq!(&row[99], "Data100");  //99 is the 100th column
  });
}
