#[macro_use]
extern crate json_in_type;

use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    let json_build = json_object!{
      "test": "abc"
    };
    assert_eq!(json_build.to_json_string(), super::BASELINE);
  });
}

pub fn serial(b: &mut Bencher) {
  b.iter(|| {
    let json_build = json_object!{
"test1": 1, "test2": 2, "test3": 3, "test4": 4, "test5": 5, "test6": 6, "test7": 7, "test8": 8, "test9": 9, "test10": 10,
"test11": 11, "test12": 12, "test13": 13, "test14": 14, "test15": 15, "test16": 16, "test17": 17, "test18": 18, "test19": 19, "test20": 20,
"test21": 21, "test22": 22, "test23": 23, "test24": 24, "test25": 25, "test26": 26, "test27": 27, "test28": 28, "test29": 29, "test30": 30,
"test31": 31, "test32": 32, "test33": 33, "test34": 34, "test35": 35, "test36": 36, "test37": 37, "test38": 38, "test39": 39, "test40": 40,
"test41": 41, "test42": 42, "test43": 43, "test44": 44, "test45": 45, "test46": 46, "test47": 47, "test48": 48, "test49": 49, "test50": 50,
"test51": 51, "test52": 52, "test53": 53, "test54": 54, "test55": 55, "test56": 56, "test57": 57, "test58": 58, "test59": 59, "test60": 60,
"test61": 61, "test62": 62, "test63": 63, "test64": 64, "test65": 65, "test66": 66, "test67": 67, "test68": 68, "test69": 69, "test70": 70,
"test71": 71, "test72": 72, "test73": 73, "test74": 74, "test75": 75, "test76": 76, "test77": 77, "test78": 78, "test79": 79, "test80": 80,
"test81": 81, "test82": 82, "test83": 83, "test84": 84, "test85": 85, "test86": 86, "test87": 87, "test88": 88, "test89": 89, "test90": 90,
"test91": 91, "test92": 92, "test93": 93, "test94": 94, "test95": 95, "test96": 96, "test97": 97, "test98": 98, "test99": 99, "test100": 100
};
    assert_eq!(json_build.to_json_string(), super::SERIAL);
  });
}

pub fn nested(b: &mut Bencher) {
  b.iter(|| {
    let json_build: json_object!{
"test1": json_object!{ "test2": json_object!{ "test3": json_object!{ "test4": json_object!{ "test5": json_object!{
"test6": json_object!{ "test7": json_object!{ "test8": json_object!{ "test9": json_object!{ "test10": json_object!{
"test11": json_object!{ "test12": json_object!{ "test13": json_object!{ "test14": json_object!{ "test15": json_object!{
"test16": json_object!{ "test17": json_object!{ "test18": json_object!{ "test19": json_object!{ "test20": json_object!{
"test21": json_object!{ "test22": json_object!{ "test23": json_object!{ "test24": json_object!{ "test25": json_object!{
"test26": json_object!{ "test27": json_object!{ "test28": json_object!{ "test29": json_object!{ "test30": json_object!{
"test31": json_object!{ "test32": json_object!{ "test33": json_object!{ "test34": json_object!{ "test35": json_object!{
"test36": json_object!{ "test37": json_object!{ "test38": json_object!{ "test39": json_object!{ "test40": json_object!{
"test41": json_object!{ "test42": json_object!{ "test43": json_object!{ "test44": json_object!{ "test45": json_object!{
"test46": json_object!{ "test47": json_object!{ "test48": json_object!{ "test49": json_object!{ "test50": json_object!{
"test51": json_object!{ "test52": json_object!{ "test53": json_object!{ "test54": json_object!{ "test55": json_object!{
"test56": json_object!{ "test57": json_object!{ "test58": json_object!{ "test59": json_object!{ "test60": json_object!{
"test61": json_object!{ "test62": json_object!{ "test63": json_object!{ "test64": json_object!{ "test65": json_object!{
"test66": json_object!{ "test67": json_object!{ "test68": json_object!{ "test69": json_object!{ "test70": json_object!{
"test71": json_object!{ "test72": json_object!{ "test73": json_object!{ "test74": json_object!{ "test75": json_object!{
"test76": json_object!{ "test77": json_object!{ "test78": json_object!{ "test79": json_object!{ "test80": json_object!{
"test81": json_object!{ "test82": json_object!{ "test83": json_object!{ "test84": json_object!{ "test85": json_object!{
"test86": json_object!{ "test87": json_object!{ "test88": json_object!{ "test89": json_object!{ "test90": json_object!{
"test91": json_object!{ "test92": json_object!{ "test93": json_object!{ "test94": json_object!{ "test95": json_object!{
"test96": json_object!{ "test97": json_object!{ "test98": json_object!{ "test99": json_object!{ "test100": json_object!{
  "result":12345
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
    };
    assert_eq!(json_build.to_json_string(), super::NESTED);
  });
}
