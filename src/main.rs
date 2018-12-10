
fn main() {
  run_bench("json_parse");
}
  
fn run_bench(func_benced: &str) {
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

  let mut write_data = String::from("# );

  //Header
  write_data += "| |";
  for (crat, _) in &crats_vec {
    write_data += &format!(" {} |", crat);
  }
  write_data += "\n";

  //Header divider
  write_data += "| --- |";
  for _ in &crats {
    write_data += " --- |";
  }
  write_data += "\n";
  
  //Data
  for (func, _) in &funcs_vec {
    write_data += &format!("| **{}** |", func);
    for (crat, _) in &crats_vec {
      if let Some(val) = map.get(&(crat, func)) {
        write_data += &format!(" {} |", (*val as f32 / 1_000.0).to_string());
      } else {
        write_data += " - |";
      }
    }
    write_data += "\n";
  }
  
  write_data += "\nSpeed units are in microseconds per iteration\n\nCompiled on:\n\n";
  
  let output = Command::new("cargo")
    .arg("version")
    .output()
    .expect("Failed to run cargo")
    .stdout;
  let output = String::from_utf8(output).unwrap();
  write_data += &output;
  
  write_data += "\nCrate versions tested:\n\n";
  
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
  
  use std::fs;
  fs::write(format!("D:\\Programming\\compare-speed\\benches\\{}\\README.md", func_benched), write_data).expect("Unable to write file");
}
