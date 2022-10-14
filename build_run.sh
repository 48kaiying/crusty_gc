#!/bin/bash
echo "building rustgc project"
cargo build
export LD_LIBRARY_PATH=./target/debug/
gcc -g ./c_app/src/application.c -o ./c_app/build/app -lrustgc -L$LD_LIBRARY_PATH
./c_app/build/app
