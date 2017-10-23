#!/bin/bash

set -e

mkdir -p build && cd build
qmake ..
make