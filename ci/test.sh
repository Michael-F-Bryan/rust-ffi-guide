#!/bin/bash

set -ex

PROJECT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $PROJECT_DIR && pwd && ls
make build
make test