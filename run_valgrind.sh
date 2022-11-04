#!/bin/bash

echo "#### RUNNING VALGRIND ON EXECUTABLE ####"
export LD_LIBRARY_PATH=./target/debug/
valgrind --leak-check=yes --show-leak-kinds=all --verbose --keep-debuginfo=yes ./c_app/build/app 