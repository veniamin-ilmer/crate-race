use bencher::Bencher;

use csv::ReaderBuilder;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = ReaderBuilder::new()
       .from_reader(super::BASELINE.as_bytes());

    let row = rdr.records().next().unwrap().unwrap();
    assert_eq!(row.get(0).unwrap(), "Data");
  });
}

pub fn rows(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = ReaderBuilder::new()
       .from_reader(super::ROWS.as_bytes());

    let row = rdr.records().nth(99).unwrap().unwrap();  //99 is the 100th row
    assert_eq!(row.get(0).unwrap(), "Row100");
  });
}

pub fn headers(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = ReaderBuilder::new()
       .from_reader(super::HEADERS.as_bytes());

    let row = rdr.records().next().unwrap().unwrap();
    assert_eq!(row.get(99).unwrap(), "Data100");  //99 is the 100th column
  });
}
