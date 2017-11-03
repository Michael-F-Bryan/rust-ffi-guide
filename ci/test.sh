#!/bin/bash

set -ex

export CTEST_OUTPUT_ON_FAILURE=1 

mkdir -p build && cd build
cmake -DCMAKE_BUILD_TYPE=Debug ..
make
make test
