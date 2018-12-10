
fn main() {
  use std::process::Command;
  let output = Command::new("cargo")
      .arg("bench")
      .arg("--bench")
      .arg("deserialize_json")
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

  use std::fs::OpenOptions;
  use std::io::prelude::*;
  use std::io::BufWriter;

  let mut file = OpenOptions::new()
      .write(true)
      .append(false)
      .open("D:\\Programming\\compare-speed\\benches\\deserialize_json\\README.md")
      .expect("Unable to write file");

  let mut stream = BufWriter::new(file);

  //Header
  stream.write(b"| |");
  for (crat, _) in &crats_vec {
    stream.write(b" ");
    stream.write(crat.as_bytes());
    stream.write(b" |");
  }
  stream.write(b"\n");

  //Header divider
  stream.write(b"| --- |");
  for _ in &crats {
    stream.write(b" --- |");
  }
  stream.write(b"\n");
  
  //Data
  for (func, _) in &funcs_vec {
    stream.write(b"| **");
    stream.write(func.as_bytes());
    stream.write(b"** |");
    for (crat, _) in &crats_vec {
      if let Some(val) = map.get(&(crat, func)) {
        stream.write(b" ");
        stream.write(val.to_string().as_bytes());
        stream.write(b" |");
      } else {
        stream.write(b" - |");
      }
    }
    stream.write(b"\n");
  }
}