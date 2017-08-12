#!/bin/bash

if command -v mdbook; then
  mdbook --version
else
  cargo install mdbook
fi

pip install --user ghp-import pytest cffi

