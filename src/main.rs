#[macro_use]
extern crate erdos;

use std::io::{self, Write};
use std::str::{self, FromStr};
use std::process::{Command, Stdio};

use erdos::dataflow::{
    operators::{JoinOperator, SourceOperator, MapOperator},
    OperatorConfig,
};
use erdos::node::Node;
use erdos::Configuration;

fn main() {
    let _s1 = connect_1_write!(
        SourceOperator,
        OperatorConfig::new().name("SourceOperator1")
    );
    let _s2 = connect_1_write!(
        SourceOperator,
        OperatorConfig::new().name("SourceOperator2")
    );
    let _s3 = connect_1_write!(JoinOperator<usize, usize, usize>, OperatorConfig::new().name("JoinOperator").node(1).arg(
        |left: Vec<usize>, right: Vec<usize>| -> usize {
            let left_sum: usize = left.iter().sum();
            let right_sum: usize = right.iter().sum();
            run_add_wasm(left_sum, right_sum).unwrap()
        }), _s1, _s2);

    // Add the mapping function as an argument to the operator via the OperatorConfig.
    let _s4 = connect_1_write!(MapOperator<usize, u64>, OperatorConfig::new().name("MapOperator").node(2).arg(
        |data: &usize| -> u64 {
            (data * 2) as u64
        }
    ), _s3);

    let args = erdos::new_app("ERDOS").get_matches();
    let mut node = Node::new(Configuration::from_args(&args));
    node.run();

}

fn run_add_wasm(left: usize, right: usize) -> io::Result<usize> {
    let mut child = Command::new("wasmedge")
        .arg("./wasm/add/pkg/add.wasm")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_fmt(format_args!("[{}, {}]", left, right))?;
    // Close stdin to finish and avoid indefinite blocking
    drop(child_stdin);
    let output = child.wait_with_output()?;
    let r = usize::from_str(str::from_utf8(&output.stdout).unwrap()).unwrap();
    Ok(r)
}
