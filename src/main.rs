/*Build a csv in this format:
  crate, version, func_benched1, func_benched2, func_benched3, ...
  
  Every day Crate Race will loop through each crate and version,
  and run `cargo search` on that crate to check if the version is the same.
  
  If it is not the same, run run_bench() on each func_benched listed.
  
  Afterwards, update the CSV to include the newest version of that crate.
  
  Phase 2 optimization:
  Keep track of the func_benched that were tested.
  If we run into another crate with a matching func_benched, don't run run_bench() on it.
  Continue with all other func_benched that were not already run.
  Continue to update the crate version in the CSV.
*/
fn main() {
  run_bench("md5");
}

/*Run benchmarks for func_benched.
  The path and everything for func_benched is configured in cargo.toml.
  Parse and sort the results, write them to README.md in the func_benched folder.
  For all crates used, run `cargo search [crate]` to get the latest crate version, and save that to the readme.
  Save the cargo/rust version into the readme too.
*/
fn run_bench(func_benched: &str) {
  use std::process::Command;
  let output = Command::new("cargo")
      .arg("bench")
      .arg("--bench")
      .arg(func_benched)
      .output()
      .expect("Failed to run cargo")
      .stdout;
  let output = String::from_utf8(output).unwrap();
  
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
    crats_vec.push((crat, sum / count));
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
    funcs_vec.push((func, sum / count));
  }
  funcs_vec.sort_by_key(|k| k.1);

  let mut write_data = format!("# {}\n", func_benched);

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
    write_data += &format!("| **{}** |", crat);
    for (func, _) in &funcs_vec {
      if let Some(val) = map.get(&(crat, func)) {
        write_data += &format!(" {} |", (*val as f32 / 1_000.0).to_string());
      } else {
        write_data += " - |";
      }
    }
    write_data += "\n";
  }
  
  write_data += "\nSpeed units are in microseconds per iteration\n\nCrate versions tested:\n\n";
  
  for (crat, _) in &crats_vec {
    let output = Command::new("cargo")
      .arg("search")
      .arg(crat)
      .output()
      .expect("Failed to run cargo")
      .stdout;
    let output = String::from_utf8(output).unwrap();
    write_data += &format!("    {}\n", output.lines().next().unwrap());
  }
  
  write_data += "\nCompiled on: `";
  
  let output = Command::new("cargo")
    .arg("version")
    .output()
    .expect("Failed to run cargo")
    .stdout;
  let output = String::from_utf8(output).unwrap();
  write_data += &output;
  
  write_data += "`";
  
  use std::fs;
  fs::write(format!("D:\\Programming\\crate-race\\benches\\{}\\README.md", func_benched), write_data).expect("Unable to write file");
}
