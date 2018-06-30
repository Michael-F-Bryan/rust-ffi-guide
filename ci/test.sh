#!/bin/bash

set -ex

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$(dirname "$SCRIPT_DIR")

cd $PROJECT_DIR && pwd && ls
make build
make test