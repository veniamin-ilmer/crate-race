use std::fs;
use std::io::{BufRead, BufReader};

mod bench_runner;

///Build a csv in this format:
///crate, version, bench1, bench2, bench3, ...
///
///Every day Crate Race will loop through each crate and version,
///and run `cargo search` on that crate to check if the version is the same.
///
///If it is not the same, run run_bench() on each bench listed.
///
///Afterwards, update the CSV to include the newest version of that crate.
///
///Optimization:
///Keep track of the benches that were tested.
///If we run into another crate with a matching bench, don't run run_bench() on it.
///Continue with all other benches that were not already run.
///Continue to update the crate version in the CSV.
///
///The first line of the code will have the cargo version listed.
///Compare this to the output of `cargo version`
///If it is different, run through all benches listed, and rerun all of them.
fn main() {
    let f = BufReader::new(
        fs::File::open("D:\\Programming\\crate-race\\crate_list.csv")
            .expect("Unable to open crate_list.csv"),
    );

    let mut benched_history = std::collections::HashSet::new();

    let mut write_data = String::new();

    println!("Checking if any Crates changed..");
    let mut new_rust_version = false;
    let mut first = true;
    for line in f.lines() {
        let line = line.expect("Unable to read line from crate_list.csv");
        if first {
            //Rust version
            first = false;
            let new_version_str = bench_runner::get_rust_version_str();
            if line != new_version_str {
                new_rust_version = true;
                println!("There is a new rust version!\nBefore:{}\nAfter: {}\nRerunning all benchmarks with new rust version...", line, new_version_str);
            }
            write_data += &format!("{}\n", new_version_str);
            continue; //Don't treat this rust line like a benchmark
        }
        let mut line_vals = line.split(",");
        if let Some(crat) = line_vals.next() {
            if let Some(old_version) = line_vals.next() {
                let crate_version_str = bench_runner::get_crate_version_str(crat);
                //Extract the version
                let mut crate_version_split = crate_version_str.split("\"");
                crate_version_split.next(); //Skip crate str
                let new_version = crate_version_split
                    .next()
                    .expect(&format!("Crate {} version unavailable!!", crat)); //Get version
                if new_version != old_version || new_rust_version {
                    //New version available! Rerun benchmarks for all benches for this crate!
                    println!(
                        "{} crate updating from {} to {}...",
                        crat, old_version, new_version
                    );
                    let mut bench_success = true;
                    let mut prepare_write_data = String::new();
                    while let Some(bench) = line_vals.next() {
                        if !benched_history.contains(bench) {
                            //Make sure to only run this bench if we didn't already run it before with another crate
                            if bench_runner::run_bench(bench) {
                                //Save it so we don't run this bench again
                                benched_history.insert(bench.to_string()); //Need to do to_string here because `bench` is borrowed from `line`.
                            } else {
                                bench_success = false;
                            }
                        }
                        prepare_write_data += &format!(",{}", bench); //Rerecord all benches into csv
                    }
                    match bench_success {
                        true => write_data += &format!("{},{}", crat, new_version), //Update csv to new version
                        false => write_data += &format!("{},{}", crat, old_version), //There was a problem, so keep the old version.
                    }
                    write_data += &prepare_write_data; //Add in all the benches, now that the crate and version were set.
                } else {
                    //Old version == new version
                    write_data += &line;
                }
            } else {
                //version not there
                write_data += &line;
            }
        } else {
            //crate not there.
            write_data += &line;
        }
        write_data += "\n";
    }
    fs::write("D:\\Programming\\crate-race\\crate_list.csv", write_data)
        .expect("Unable to write file");
    println!("Done");
}
