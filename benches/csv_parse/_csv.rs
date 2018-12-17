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

    let row = rdr.records().nth(999).unwrap().unwrap();  //999 is the 1000th row
    assert_eq!(&row[0], "Row1000");
  });
}

pub fn headers(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = ReaderBuilder::new()
       .from_reader(super::HEADERS.as_bytes());

    let row = rdr.records().next().unwrap().unwrap();
    assert_eq!(&row[999], "Data1000");  //999 is the 1000th column
  });
}
