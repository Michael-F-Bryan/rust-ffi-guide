#!/bin/bash
set -ex

BOOK_DIR=build/book

# TODO: remove me when PR #41 is merged
command -v mdbook >/dev/null 2>&1 || cargo install mdbook
command -v aws >/dev/null 2>&1 || pip3 install awscli
mdbook build 
aws s3 sync $BOOK_DIR s3://temp.michaelfbryan.com

# Only upload the built book to github pages if it's a commit to master
if [ "$TRAVIS_BRANCH" = master -a "$TRAVIS_PULL_REQUEST" = false ]; then
  mdbook build 
  ghp-import -n $BOOK_DIR 
  git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
fi
