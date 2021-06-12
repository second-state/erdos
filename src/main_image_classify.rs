#[macro_use]
extern crate erdos;

use std::fs;
use std::io::{self, Write};
use std::str::{self, FromStr};
use std::process::{Command, Stdio};

use erdos::dataflow::{
    message::{Timestamp, Message},
    operators::{JoinOperator, SourceOperator, MapOperator},
    stream::{IngestStream, ExtractStream},
    OperatorConfig,
};
use erdos::node::Node;
use erdos::Configuration;

fn main() {
    let mut ingest_stream = IngestStream::new(0);

    // Add the mapping function as an argument to the operator via the OperatorConfig.
    let r_stream = connect_1_write!(MapOperator<Vec<u8>, String>, OperatorConfig::new().name("MapOperator").arg(
        |data: &Vec<u8>| -> String {
            run_classify_wasm(data)
        }
    ), ingest_stream);

    let mut extract_stream = ExtractStream::new(0, &r_stream);

    let args = erdos::new_app("ERDOS").get_matches();
    let mut node = Node::new(Configuration::from_args(&args));
    node.run_async();

    let foods = vec!["hamburger.jpg", "hotdog.jpg", "sanwich.jpg"];

    for (i, food) in foods.iter().enumerate() {
        let ts = Timestamp::new(vec![i as u64]);
        let fv = fs::read(format!("./images/{}", food)).unwrap();
        match ingest_stream.send(Message::new_message(ts.clone(), fv)) {
            Err(e) => (),
            _ => (),
        };
        match ingest_stream.send(Message::new_watermark(ts.clone())) {
            Err(e) => (),
            _ => (),
        };
        let msg = extract_stream.read().unwrap();
        println!("<>-----{:?}", msg.data().unwrap());
        extract_stream.read().unwrap();
    }
}

fn run_classify_wasm(bytes: &Vec<u8>) -> String {
    let mut child = Command::new("ssvm-tensorflow")
        .arg("./wasm/food_classify/pkg/food_classify.wasm")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write(bytes).unwrap();
    // Close stdin to finish and avoid indefinite blocking
    drop(child_stdin);
    let output = child.wait_with_output().unwrap();
    String::from(str::from_utf8(&output.stdout).unwrap())
}
