static BENCHES_DIR: &str = "D:\\Programming\\crate-race\\benches";

use std::collections::{HashMap, HashSet};
use std::process::Command;
use std::fs;
use std::io::{BufRead, BufReader};

///Run benchmarks for func_benched.
///The path and everything for func_benched is configured in cargo.toml.
///Parse and sort the results, write them to README.md in the func_benched folder.
///For all crates used, run `cargo search [crate]` to get the latest crate version, and save that to the readme.
///Save the cargo/rust version into the readme too.
pub fn run_bench(func_benched: &str) -> bool {
    let bench_output = Command::new("cargo")
        .arg("bench")
        .arg("--bench")
        .arg(func_benched)
        .output()
        .expect("Failed to run cargo");
    if !bench_output.status.success() {
        println!("*** Benchmark failed: {} ***", func_benched);
        return false;
    }

    let output = String::from_utf8(bench_output.stdout).unwrap();

    //map = Key: (crate, function), Value: Speed
    //crats = List of crates involved with this benchmark
    //funcs = List of functions involved with this benchmark

    let (map, crats, funcs) = build_hashes(&output);

    let crats_sorted = build_sorted_crate_vec(&map, &crats, &funcs);
    let funcs_sorted = build_sorted_func_vec(&map, &crats, &funcs);
    let fastest_crat = get_fastest_crat_per_func(&map, &crats, &funcs);

    let mut write_data = format!("# {}\n", func_benched); //Header of README

    //Copy a description of each func from bench.rs comments
    let f = fs::File::open(format!("{}\\{}\\bench.rs", BENCHES_DIR, func_benched))
        .expect("Unable to open bench.rs to read comments");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line from bench.rs to read comments");
        if line.len() >= 3 && &line[0..3] == "//!" {
            write_data += &format!("{}\n", &line[3..]); //Remove "//!"
        }
    }
    write_data += "\n";

    //Header
    write_data += "| |";
    for (func, _) in &funcs_sorted {
        write_data += &format!(" {} |", func);
    }
    write_data += "\n";

    //Header divider
    write_data += "| --- |";
    for _ in &funcs_sorted {
        write_data += " --- |";
    }
    write_data += "\n";

    //Data
    for (crat, _) in &crats_sorted {
        write_data += &format!("| **[{}](https://crates.io/crates/{})** |", crat, crat);
        for (func, _) in &funcs_sorted {
            if let Some(val) = map.get(&(crat, func)) {
                if fastest_crat.get(func) == Some(crat) {
                    //Do we need to bolden it?
                    write_data += &format!(" *{}* |", (*val as f32 / 1_000.0).to_string());
                } else {
                    write_data += &format!(" {} |", (*val as f32 / 1_000.0).to_string());
                }
            } else {
                write_data += " - |"; //Missing data
            }
        }
        write_data += "\n";
    }
    write_data += "\nSpeed units are in microseconds per iteration. Less is better.\n";

    let related_benches = find_related_branches(&crats, func_benched);
    if related_benches.len() > 0 {
        write_data += "\n## Related Functions\n\n";
        for bench in related_benches {
            write_data += &format!("* [{}](../{})\n", bench, bench);
        }
    }
    
    write_data += "\n## Crate versions\n\n";
    for (crat, _) in &crats_sorted {
        write_data += &format!("    {}\n", get_crate_version_str(crat));
    }

    write_data += &format!("\nCompiled on: `{}`", &get_cargo_version_str());
    
    fs::write(
        format!("{}\\{}\\README.md", BENCHES_DIR, func_benched),
        write_data,
    )
    .expect("Unable to write file");

    true
}

fn build_hashes(output: &str) -> (HashMap<(&str, &str), u32>, HashSet<&str>, HashSet<&str>) {
    use regex::Regex;
    let re = Regex::new(r"test _(.+)::([^\s]+)\s+... bench:.\s+([^\s]+) ns/iter.*").unwrap();

    let mut map = HashMap::new();
    let mut crats = HashSet::new();
    let mut funcs = HashSet::new();

    for cap in re.captures_iter(output) {
        if let (Some(crat), Some(func), Some(speed)) =
            (cap.iter().nth(1), cap.iter().nth(2), cap.iter().nth(3))
        {
            if let (Some(crat), Some(func), Some(speed)) = (crat, func, speed) {
                let speed = speed.as_str().replace(",", "");
                if let Ok(speed_int) = speed.parse::<u32>() {
                    crats.insert(crat.as_str());
                    funcs.insert(func.as_str());
                    map.insert((crat.as_str(), func.as_str()), speed_int);
                }
            }
        }
    }
    (map, crats, funcs)
}

fn build_sorted_crate_vec<'c>(
    map: &HashMap<(&str, &str), u32>,
    crats: &HashSet<&'c str>,
    funcs: &HashSet<&str>,
) -> Vec<(&'c str, u32)> {
    let mut crats_sorted = Vec::new();
    for crat in crats {
        let mut sum = 0;
        let mut count = 0;
        for func in funcs {
            if let Some(val) = map.get(&(crat, func)) {
                sum += val;
                count += 1;
            }
        }
        crats_sorted.push((*crat, sum / count)); //Save the average
    }
    crats_sorted.sort_by_key(|k| k.1);
    crats_sorted
}

fn build_sorted_func_vec<'f>(
    map: &HashMap<(&str, &str), u32>,
    crats: &HashSet<&str>,
    funcs: &HashSet<&'f str>,
) -> Vec<(&'f str, u32)> {
    let mut funcs_sorted = Vec::new();
    for func in funcs {
        let mut sum = 0;
        let mut count = 0;
        for crat in crats {
            if let Some(val) = map.get(&(crat, func)) {
                sum += val;
                count += 1;
            }
        }
        funcs_sorted.push((*func, sum / count)); //Save the average
    }
    funcs_sorted.sort_by_key(|k| k.1);
    funcs_sorted
}

///Save the smallest values per function to embolden later
fn get_fastest_crat_per_func<'f, 'c>(
    map: &HashMap<(&str, &str), u32>,
    crats: &HashSet<&'c str>,
    funcs: &HashSet<&'f str>,
) -> HashMap<&'f str, &'c str> {
    let mut fastest_crat = HashMap::new();
    for func in funcs {
        let mut smallest = u32::max_value();
        let mut smallest_crat = "";
        for crat in crats {
            if let Some(val) = map.get(&(crat, func)) {
                if *val < smallest {
                    smallest = *val;
                    smallest_crat = crat;
                }
            }
        }
        fastest_crat.insert(*func, smallest_crat); //Save the smallest
    }
    fastest_crat
}

///Find any related benches.
fn find_related_branches(crats: &HashSet<&str>, func_benched: &str) -> HashSet<String> {
    let mut related_benches = HashSet::new();

    let f = BufReader::new(
        fs::File::open("D:\\Programming\\crate-race\\crate_list.csv")
            .expect("Unable to open crate_list.csv"),
    );
    for line in f.lines() {
        let line = line.expect("Unable to read line from crate_list.csv in run_bench function");
        let mut line_vals = line.split(",");
        if let Some(crat) = line_vals.next() {
            if crats.contains(crat) {
                line_vals.next(); //Skip version
                while let Some(bench) = line_vals.next() {
                    if bench != func_benched {
                        related_benches.insert(bench.to_string());
                    }
                }
            }
        }
    }
    related_benches
}

pub fn get_crate_version_str(crat: &str) -> String {
    let output = Command::new("cargo")
        .arg("search")
        .arg(crat)
        .output()
        .expect("Failed to run cargo search")
        .stdout;
    String::from_utf8(output)
        .expect("Failed to get output from cargo search")
        .lines()
        .next()
        .expect("Failed to read the first line of cargo search")
        .to_string()
}

pub fn get_cargo_version_str() -> String {
    let output = Command::new("cargo")
        .arg("version")
        .output()
        .expect("Failed to run cargo version")
        .stdout;
    let output = String::from_utf8(output).expect("Failed to get output from cargo version");
    output.trim().to_string()
}
