///Baseline: Simple json with just one entry.
///Serial: Opposite of nested. 100 entries listed in a "flat" format, one by one, without any tree structure.
///Nested: 100 nested entries, each inside of the other.

#[macro_use]
extern crate bencher;

static BASELINE: &str = r#"{"test":"abc"}"#;

static SERIAL: &str = r#"{"test1":1,"test2":2,"test3":3,"test4":4,"test5":5,"test6":6,"test7":7,"test8":8,"test9":9,"test10":10,"test11":11,"test12":12,"test13":13,"test14":14,"test15":15,"test16":16,"test17":17,"test18":18,"test19":19,"test20":20,"test21":21,"test22":22,"test23":23,"test24":24,"test25":25,"test26":26,"test27":27,"test28":28,"test29":29,"test30":30,"test31":31,"test32":32,"test33":33,"test34":34,"test35":35,"test36":36,"test37":37,"test38":38,"test39":39,"test40":40,"test41":41,"test42":42,"test43":43,"test44":44,"test45":45,"test46":46,"test47":47,"test48":48,"test49":49,"test50":50,"test51":51,"test52":52,"test53":53,"test54":54,"test55":55,"test56":56,"test57":57,"test58":58,"test59":59,"test60":60,"test61":61,"test62":62,"test63":63,"test64":64,"test65":65,"test66":66,"test67":67,"test68":68,"test69":69,"test70":70,"test71":71,"test72":72,"test73":73,"test74":74,"test75":75,"test76":76,"test77":77,"test78":78,"test79":79,"test80":80,"test81":81,"test82":82,"test83":83,"test84":84,"test85":85,"test86":86,"test87":87,"test88":88,"test89":89,"test90":90,"test91":91,"test92":92,"test93":93,"test94":94,"test95":95,"test96":96,"test97":97,"test98":98,"test99":99,"test100":100}"#;

static NESTED: &str = r#"{"test1":{"test2":{"test3":{"test4":{"test5":{"test6":{"test7":{"test8":{"test9":{"test10":{"test11":{"test12":{"test13":{"test14":{"test15":{"test16":{"test17":{"test18":{"test19":{"test20":{"test21":{"test22":{"test23":{"test24":{"test25":{"test26":{"test27":{"test28":{"test29":{"test30":{"test31":{"test32":{"test33":{"test34":{"test35":{"test36":{"test37":{"test38":{"test39":{"test40":{"test41":{"test42":{"test43":{"test44":{"test45":{"test46":{"test47":{"test48":{"test49":{"test50":{"test51":{"test52":{"test53":{"test54":{"test55":{"test56":{"test57":{"test58":{"test59":{"test60":{"test61":{"test62":{"test63":{"test64":{"test65":{"test66":{"test67":{"test68":{"test69":{"test70":{"test71":{"test72":{"test73":{"test74":{"test75":{"test76":{"test77":{"test78":{"test79":{"test80":{"test81":{"test82":{"test83":{"test84":{"test85":{"test86":{"test87":{"test88":{"test89":{"test90":{"test91":{"test92":{"test93":{"test94":{"test95":{"test96":{"test97":{"test98":{"test99":{"test100":{"result":12345}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}"#;

mod _json;
mod _serde_json;
mod _json_in_type;

benchmark_group!(baseline, _json::baseline, _serde_json::baseline, _json_in_type::baseline);
benchmark_group!(serial, _json::serial, _serde_json::serial, _json_in_type::serial);
benchmark_group!(nested, _json::nested, _serde_json::nested, _json_in_type::nested);

benchmark_main!(baseline, serial, nested);
