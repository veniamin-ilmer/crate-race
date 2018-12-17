use std::fs;
use std::io::{BufRead, BufReader};
use std::process::Command;

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
  let f = fs::File::open("D:\\Programming\\crate-race\\crate_list.csv")
                   .expect("Unable to open crate_list.csv");
  let f = BufReader::new(f);

  let mut benched_history = std::collections::HashSet::new();
  
  let mut write_data = String::new();
  
  println!("Checking if any Crates changed..");
  let mut new_cargo_version = false;
  let mut first = true;
  for line in f.lines() {
    let line = line.expect("Unable to read line from crate_list.csv");
    if first { //Cargo version
      first = false;
      let new_version_str = get_cargo_version_str();
      if line != new_version_str {
        new_cargo_version = true;
        println!("There is a new cargo version!\nBefore:{}\nAfter: {}\nRerunning all benchmarks with new cargo version...", line, new_version_str);
      }
      write_data += &format!("{}\n", new_version_str);
      continue; //Don't treat this cargo line like a benchmark
    }
    let mut line_vals = line.split(",");
    if let Some(crat) = line_vals.next() {
      if let Some(old_version) = line_vals.next() {
        let crate_version_str = get_crate_version_str(crat);
        //Extract the version
        let mut crate_version_split = crate_version_str.split("\"");
        crate_version_split.next(); //Skip crate str
        let new_version = crate_version_split.next().expect(&format!("Crate {} version unavailable!!", crat)); //Get version
        if new_version != old_version || new_cargo_version { //New version available! Rerun benchmarks for all benches for this crate!
          println!("{} crate updating from {} to {}...", crat, old_version, new_version);
          let mut bench_success = true;
          let mut prepare_write_data = String::new();
          while let Some(bench) = line_vals.next() {
            if !benched_history.contains(bench) {  //Make sure to only run this bench if we didn't already run it before with another crate
              if run_bench(bench) {
                //Save it so we don't run this bench again
                benched_history.insert(bench.to_string());  //Need to do to_string here because `bench` is borrowed from `line`.
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
          write_data += &prepare_write_data;  //Add in all the benches, now that the crate annd verrsion were set.

        } else {  //Old version == new version
          write_data += &line;
        }
      } else {  //version not there
        write_data += &line;
      }
    } else {  //crate not there.
      write_data += &line;
    }
    write_data += "\n";
  }
  fs::write("D:\\Programming\\crate-race\\crate_list.csv", write_data).expect("Unable to write file");
  println!("Done");
  
}

///Run benchmarks for func_benched.
///The path and everything for func_benched is configured in cargo.toml.
///Parse and sort the results, write them to README.md in the func_benched folder.
///For all crates used, run `cargo search [crate]` to get the latest crate version, and save that to the readme.
///Save the cargo/rust version into the readme too.

fn run_bench(func_benched: &str) -> bool {
  let bench_output = Command::new("cargo")
      .arg("bench").arg("--bench").arg(func_benched)
      .output().expect("Failed to run cargo");
  if !bench_output.status.success() {
    println!("*** Benchmark failed: {} ***", func_benched);
    return false;
  }
  let output = String::from_utf8(bench_output.stdout).unwrap();
  
  use regex::Regex;
  let re = Regex::new(r"test _(.+)::([^\s]+)\s+... bench:.\s+([^\s]+) ns/iter.*").unwrap();
  
  use std::collections::{HashMap, HashSet};
  let mut map = HashMap::new();
  let mut crats = HashSet::new();
  let mut funcs = HashSet::new();
  
  for cap in re.captures_iter(&output) {
    if let (Some(crat), Some(func), Some(speed)) = (cap.iter().nth(1), cap.iter().nth(2), cap.iter().nth(3)) {
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

  //Sort the crates
  let mut crats_vec = Vec::new();
  for crat in &crats {
    let mut sum = 0;
    let mut count = 0;
    for func in &funcs {
      if let Some(val) = map.get(&(crat, func)) {
        sum += val;
        count += 1;
      }
    }
    crats_vec.push((crat, sum / count));  //Save the average
  }
  crats_vec.sort_by_key(|k| k.1);

  //Sort the functions
  let mut funcs_vec = Vec::new();
  for func in &funcs {
    let mut sum = 0;
    let mut count = 0;
    for crat in &crats {
      if let Some(val) = map.get(&(crat, func)) {
        sum += val;
        count += 1;
      }
    }
    funcs_vec.push((func, sum / count));  //Save the average
  }
  funcs_vec.sort_by_key(|k| k.1);
  
  //Save the smallest values per function to embolden later
  let mut smallest_map = HashMap::new();
  for func in &funcs {
    let mut smallest = u32::max_value();
    let mut smallest_crat = "";
    for crat in &crats {
      if let Some(val) = map.get(&(crat, func)) {
        if *val < smallest {
          smallest = *val;
          smallest_crat = crat;
        }
      }
    }
    smallest_map.insert(func, smallest_crat);  //Save the smallest
  }

  let mut write_data = format!("# {}\n", func_benched);

  //Copy a description of each func from bench.rs comments
  let f = fs::File::open(format!("D:\\Programming\\crate-race\\benches\\{}\\bench.rs", func_benched))
               .expect("Unable to open bench.rs to read comments");
  let f = BufReader::new(f);

  for line in f.lines() {
    let line = line.expect("Unable to read line from bench.rs to read comments");
    if line.len() >= 3 && &line[0..3] == "///" {
      write_data += &line[3..];
      write_data += "\n";
    }
  }
  write_data += "\n";

  //Header
  write_data += "| |";
  for (func, _) in &funcs_vec {
    write_data += &format!(" {} |", func);
  }
  write_data += "\n";

  //Header divider
  write_data += "| --- |";
  for _ in &funcs_vec {
    write_data += " --- |";
  }
  write_data += "\n";
  
  //Data
  for (crat, _) in &crats_vec {
    write_data += &format!("| **[{}](https://crates.io/crates/{})** |", crat, crat);
    for (func, _) in &funcs_vec {
      if let Some(val) = map.get(&(crat, func)) {
        if smallest_map.get(func) == Some(crat) {  //Do we need to bolden it?
          write_data += &format!(" *{}* |", (*val as f32 / 1_000.0).to_string());
        } else {
          write_data += &format!(" {} |", (*val as f32 / 1_000.0).to_string());
        }
      } else {
        write_data += " - |";
      }
    }
    write_data += "\n";
  }
  
  write_data += "\nSpeed units are in microseconds per iteration. Less is better.\n\n## Crate versions\n\n";
  
  for (crat, _) in &crats_vec {
    write_data += &format!("    {}\n", get_crate_version_str(crat));
  }
  
  write_data += "\nCompiled on: `";
  write_data += &get_cargo_version_str();
  write_data += "`";
  
  fs::write(format!("D:\\Programming\\crate-race\\benches\\{}\\README.md", func_benched), write_data).expect("Unable to write file");
  
  true
}

fn get_cargo_version_str() -> String {
  let output = Command::new("cargo")
    .arg("version")
    .output().expect("Failed to run cargo version").stdout;
  let output = String::from_utf8(output).expect("Failed to get output from cargo version");
  output.trim().to_string()
}

fn get_crate_version_str(crat: &str) -> String {
  let output = Command::new("cargo")
    .arg("search").arg(crat)
    .output().expect("Failed to run cargo search").stdout;
  String::from_utf8(output)
         .expect("Failed to get output from cargo search")
         .lines().next().expect("Failed to read the first line of cargo search")
         .to_string()
}
