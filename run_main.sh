#!/bin/bash
addrs="--data-addresses 127.0.0.1:9000,127.0.0.1:9001,127.0.0.1:9002 --control-addresses 127.0.0.1:9003,127.0.0.1:9004,127.0.0.1:9005"
( ./target/debug/erdos $addrs 2>&1 & echo $! >&3 ) 3>node0 | awk '{ print "[node-0]", $0 }' &
( ./target/debug/erdos --index 1 $addrs 2>&1 & echo $! >&3 ) 3>node1 | awk '{ print "[node-1]", $0 }' &
trap 'kill $(<node0); kill $(<node1); kill $(<node2); rm node0; rm node1; rm node2' SIGINT
( ./target/debug/erdos --index 2 $addrs 2>&1 & echo $! >&3 ) 3>node2 | awk '{ print "[node-2]", $0 }'
