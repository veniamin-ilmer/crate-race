#[macro_use]
extern crate bencher;

static BASELINE: &str = "10329978488239059262599702099394727095397746340117372869212250571234293987594703124871765375385424468563282236864226607350415360000000000000000000000";

mod _json;
mod _serde_json;
mod _json5;

benchmark_group!(baseline, _json::baseline, _serde_json::baseline, _json5::baseline);
benchmark_group!(serial, _json::serial, _serde_json::serial, _json5::serial);
benchmark_group!(nested, _json::nested, _serde_json::nested, _json5::nested);

benchmark_main!(baseline, serial, nested);
