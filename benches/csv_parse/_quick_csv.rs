use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = quick_csv::Csv::from_reader(super::BASELINE.as_bytes())
        .has_header(true);

    let row = rdr.next().unwrap().unwrap();
    assert_eq!(row.columns().unwrap().next().unwrap(), "Data");
  });
}

pub fn rows(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = quick_csv::Csv::from_reader(super::ROWS.as_bytes())
        .has_header(true);

    let row = rdr.nth(999).unwrap().unwrap();  //999 is the 1000th row
    assert_eq!(row.columns().unwrap().next().unwrap(), "Row1000");
  });
}

/*This fails. Some bug with quick csv handling big headers
pub fn headers(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = quick_csv::Csv::from_reader(super::HEADERS.as_bytes())
        .has_header(true);

    let row = rdr.next().unwrap().unwrap();
    assert_eq!(row.columns().unwrap().nth(999).unwrap(), "Data1000");  //999 is the 100th column
  });
}
*/