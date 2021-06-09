use std::io::{self, Read};
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;

fn main() {
  _initialize();
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).expect("Error reading from STDIN");
  let args: Vec<i32> = serde_json::from_str(&buffer).unwrap();
  print!("{:?}", args[0] + args[1]);
}

