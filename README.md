# Using `unsafe` for Fun and Profit


[![Build Status](https://travis-ci.org/Michael-F-Bryan/rust-ffi-guide.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/rust-ffi-guide)

A guide to traversing the FFI boundary between Rust and other languages. A
rendered version is available [here][gh-pages]. This guide is centred around the
idea of building a REST client using `Qt` (C++) for the GUI and `reqwest` (Rust)
for the business logic.


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
$ mdbook serve --open
```

It should now be viewable at [http://localhost:3000/](http://localhost:3000/) 
(if it didn't open up automatically).

To build the application itself you'll need the following installed:

- qt5
- rust (install with [rustup])
- mdbook (`cargo install mdbook`)

In this application we're using `cmake` as the build system. The
`ci/build.sh` script will make a `build/` directory and invoke `cmake` to 
compile everything.

```
$ ./ci/build.sh
```

The final application should now be at `build/rest_client`.

Alternatively, if you don't want to install all the dependencies I've created a
docker image ([michaelfbryan/ffi-guide][docker]) for compiling Rust and Qt.

```
$ docker run -v $(pwd):/code --user $UID michaelfbryan/ffi-guide ci/build.sh
```


## Contributing

If there's anything you feel is missing or could be
improved, please [create an issue][issues]. Pull requests are welcome too!

The repository has been designed so that each chapter has its own `Makefile`
which defines three rules, `build`, `test`, and `clean`. This allows you to run 
`make test` in the the top level directory and `make` will iterate through each
chapter, building and testing the examples. As such, changes which add new
files may need to update the chapter's `Makefile` so that it is tested
automatically.


### Contributors

- [Michael-F-Bryan](https://github.com/Michael-F-Bryan)
- [gurgalex](https://github.com/gurgalex)


[gh-pages]: https://michael-f-bryan.github.io/rust-ffi-guide/
[issues]: https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/new
[rustup]: https://rustup.rs/
[docker]: https://hub.docker.com/r/michaelfbryan/ffi-guide/