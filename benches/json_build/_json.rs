#[macro_use]
extern crate json;

use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let json_build = object!{
      "test" => "abc"
    };
    assert_eq!(json_build.dump(), super::BASELINE);
  });
}

pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let json_build = object!{
      "test1" => 1,
      "test2" => 2,
      "test3" => 3,
      "test4" => 4,
      "test5" => 5,
      "test6" => 6,
      "test7" => 7,
      "test8" => 8,
      "test9" => 9,
      "test10" => 10,
      "test11" => 11,
      "test12" => 12,
      "test13" => 13,
      "test14" => 14,
      "test15" => 15,
      "test16" => 16,
      "test17" => 17,
      "test18" => 18,
      "test19" => 19,
      "test20" => 20,
      "test21" => 21,
      "test22" => 22,
      "test23" => 23,
      "test24" => 24,
      "test25" => 25,
      "test26" => 26,
      "test27" => 27,
      "test28" => 28,
      "test29" => 29,
      "test30" => 30,
      "test31" => 31,
      "test32" => 32,
      "test33" => 33,
      "test34" => 34,
      "test35" => 35,
      "test36" => 36,
      "test37" => 37,
      "test38" => 38,
      "test39" => 39,
      "test40" => 40,
      "test41" => 41,
      "test42" => 42,
      "test43" => 43,
      "test44" => 44,
      "test45" => 45,
      "test46" => 46,
      "test47" => 47,
      "test48" => 48,
      "test49" => 49,
      "test50" => 50,
      "test51" => 51,
      "test52" => 52,
      "test53" => 53,
      "test54" => 54,
      "test55" => 55,
      "test56" => 56,
      "test57" => 57,
      "test58" => 58,
      "test59" => 59,
      "test60" => 60,
      "test61" => 61,
      "test62" => 62,
      "test63" => 63,
      "test64" => 64,
      "test65" => 65,
      "test66" => 66,
      "test67" => 67,
      "test68" => 68,
      "test69" => 69,
      "test70" => 70,
      "test71" => 71,
      "test72" => 72,
      "test73" => 73,
      "test74" => 74,
      "test75" => 75,
      "test76" => 76,
      "test77" => 77,
      "test78" => 78,
      "test79" => 79,
      "test80" => 80,
      "test81" => 81,
      "test82" => 82,
      "test83" => 83,
      "test84" => 84,
      "test85" => 85,
      "test86" => 86,
      "test87" => 87,
      "test88" => 88,
      "test89" => 89,
      "test90" => 90,
      "test91" => 91,
      "test92" => 92,
      "test93" => 93,
      "test94" => 94,
      "test95" => 95,
      "test96" => 96,
      "test97" => 97,
      "test98" => 98,
      "test99" => 99,
      "test100" => 100
    };
    assert_eq!(json_build.dump(), super::SERIAL);
  });
}

pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let json_build = object!{
      "test1" => object!{
      "test2" => object!{
      "test3" => object!{
      "test4" => object!{
      "test5" => object!{
      "test6" => object!{
      "test7" => object!{
      "test8" => object!{
      "test9" => object!{
      "test10" => object!{
      "test11" => object!{
      "test12" => object!{
      "test13" => object!{
      "test14" => object!{
      "test15" => object!{
      "test16" => object!{
      "test17" => object!{
      "test18" => object!{
      "test19" => object!{
      "test20" => object!{
      "test21" => object!{
      "test22" => object!{
      "test23" => object!{
      "test24" => object!{
      "test25" => object!{
      "test26" => object!{
      "test27" => object!{
      "test28" => object!{
      "test29" => object!{
      "test30" => object!{
      "test31" => object!{
      "test32" => object!{
      "test33" => object!{
      "test34" => object!{
      "test35" => object!{
      "test36" => object!{
      "test37" => object!{
      "test38" => object!{
      "test39" => object!{
      "test40" => object!{
      "test41" => object!{
      "test42" => object!{
      "test43" => object!{
      "test44" => object!{
      "test45" => object!{
      "test46" => object!{
      "test47" => object!{
      "test48" => object!{
      "test49" => object!{
      "test50" => object!{
      "test51" => object!{
      "test52" => object!{
      "test53" => object!{
      "test54" => object!{
      "test55" => object!{
      "test56" => object!{
      "test57" => object!{
      "test58" => object!{
      "test59" => object!{
      "test60" => object!{
      "test61" => object!{
      "test62" => object!{
      "test63" => object!{
      "test64" => object!{
      "test65" => object!{
      "test66" => object!{
      "test67" => object!{
      "test68" => object!{
      "test69" => object!{
      "test70" => object!{
      "test71" => object!{
      "test72" => object!{
      "test73" => object!{
      "test74" => object!{
      "test75" => object!{
      "test76" => object!{
      "test77" => object!{
      "test78" => object!{
      "test79" => object!{
      "test80" => object!{
      "test81" => object!{
      "test82" => object!{
      "test83" => object!{
      "test84" => object!{
      "test85" => object!{
      "test86" => object!{
      "test87" => object!{
      "test88" => object!{
      "test89" => object!{
      "test90" => object!{
      "test91" => object!{
      "test92" => object!{
      "test93" => object!{
      "test94" => object!{
      "test95" => object!{
      "test96" => object!{
      "test97" => object!{
      "test98" => object!{
      "test99" => object!{
      "test100" => object!{
      "result":12345
      }}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
    };
    assert_eq!(json_build.dump(), super::NESTED);
  });
}
