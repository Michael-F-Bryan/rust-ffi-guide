# Linking and Building

Up until now we've mostly been invoking `rustc` directly from the terminal,
compiling and linking from source and deliberately trying to keep things
simple. Unfortunately writing code in the real world is a bit more complicated,
forcing you to deal with different build systems, system libraries,
pre-compiled binaries, and import libraries (just to name a few).

These all work together to make it a massive pain to *just get your code to
compile*.

> **TODO:** Mention the following:
>
> - Static linking vs dynamic linking
> - *Import libraries* on Windows
> - Cargo build scripts
> - Linking to system libraries
> - Using pre-compiled artefacts instead of compiling from source
