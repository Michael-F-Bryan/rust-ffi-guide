# Abusing `unsafe` for Fun and Profit


[![Build Status](https://travis-ci.org/Michael-F-Bryan/rust-ffi-guide.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/rust-ffi-guide)

A guide to traversing the FFI boundary between Rust and other languages. A 
rendered version is automatically rendered every time I push a commit and is
viewable at [https://michael-f-bryan.github.io/rust-ffi-guide/][gh-pages].


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

If there's anything you want me to add or you notice something which might need
changing, [make an issue][issues]. Pull requests are welcome too!


[gh-pages]: https://michael-f-bryan.github.io/rust-ffi-guide/
[issues]: https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/new
