#!/bin/bash

echo "#### Building Rust Libraries ####"
cargo build
echo "Success"
echo "#### Building C Application ####"
export LD_LIBRARY_PATH=./target/debug/
mkdir -p ./c_app/build

# Comment out for verbose info on linker
    # gcc -g ./c_app/src/application.c -o ./c_app/build/app -lrustgc -L$LD_LIBRARY_PATH --verbose
gcc -g ./c_app/src/application.c -o ./c_app/build/app -lrustgc -L$LD_LIBRARY_PATH

echo "Success"
echo "#### Running ####"
./c_app/build/app
