#!/bin/bash

echo "#### Building Rust Libraries ####"
cargo build
echo "Success"
echo "#### Building C Application ####"
export LD_LIBRARY_PATH=./target/debug/
gcc -g ./c_app/src/application.c -o ./c_app/build/app -lrustgc -L$LD_LIBRARY_PATH
echo "Success"
echo "#### Running ####"
./c_app/build/app
