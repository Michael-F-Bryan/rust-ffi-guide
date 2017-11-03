#!/bin/bash
set -ex

BOOK_DIR=build/book

# Only upload the built book to github pages if it's a commit to master
if [ "$TRAVIS_BRANCH" = master -a "$TRAVIS_PULL_REQUEST" = false ]; then
  mdbook build 
  ls 
  git status
  git branch
  ghp-import -n $BOOK_DIR 
  git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
fi
