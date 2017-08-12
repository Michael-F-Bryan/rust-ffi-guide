# Using `unsafe` for Fun and Profit


[![Build Status](https://travis-ci.org/Michael-F-Bryan/rust-ffi-guide.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/rust-ffi-guide)

A guide to traversing the FFI boundary between Rust and other languages. A
rendered version is available [here][gh-pages].


## Building

Building and viewing the book locally is really easy. First you need to get the
source code:

```bash
$ git clone https://github.com/Michael-F-Bryan/rust-ffi-guide
```

Make sure you have `mdbook` installed:

```bash
$ cargo install mdbook
```

Then tell `mdbook` to build and serve the book:

```bash
$ mdbook serve
```

It should now be viewable at [http://localhost:3000/](http://localhost:3000/).


## Contributing

If there's anything you feel is missing or could be
improved, please [create an issue][issues]. Pull requests are welcome too!

The repository has been designed so that each chapter has its own `Makefile`
which defines three rules, `build`, `test`, and `clean`. This allows you to run 
`make test` in the the top level directory and `make` will iterate through each
chapter, building and testing the examples. As such, changes which add new
files may need to update the chapter's `Makefile` so that it is tested
automatically.


[gh-pages]: https://michael-f-bryan.github.io/rust-ffi-guide/
[issues]: https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/new
