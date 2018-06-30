#!/bin/bash
set -ex

# Make sure mdbook is installed
command -v mdbook >/dev/null 2>&1 || cargo install --debug mdbook

mdbook build