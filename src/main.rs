
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
  let re = Regex::new(r"test _(.+)::([^\s]+)\s+... bench:.\s+([^\s]+ ns/iter).*").unwrap();
  for cap in re.captures_iter(&output) {
    println!("Crate: {} Function: {} Speed: {}", &cap[1], &cap[2], &cap[3]);
  }
}