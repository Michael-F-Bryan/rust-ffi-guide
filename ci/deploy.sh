#!/bin/bash
set -x

# Only upload the built book to github pages if it's a commit to master
if [ "$TRAVIS_BRANCH" = master -a "$TRAVIS_PULL_REQUEST" = false ]; then
  mdbook build 
  ghp-import -n book 
  git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
fi
