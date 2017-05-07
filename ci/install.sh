#!/bin/bash

if command -v mdbook; then
  mdbook --version
else
  cargo install mdbook
fi

pip install --user ghp-import
sudo apt-get install -y clang bzip2
