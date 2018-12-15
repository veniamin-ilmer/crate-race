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

    let row = rdr.nth(99).unwrap().unwrap();  //99 is the 100th row
    assert_eq!(row.columns().unwrap().next().unwrap(), "Row100");
  });
}

pub fn headers(b: &mut Bencher) {
  b.iter(|| {
    let mut rdr = quick_csv::Csv::from_reader(super::HEADERS.as_bytes())
        .has_header(true);

    let row = rdr.next().unwrap().unwrap();
    assert_eq!(row.columns().unwrap().nth(99).unwrap(), "Data100");  //99 is the 100th column
  });
}
