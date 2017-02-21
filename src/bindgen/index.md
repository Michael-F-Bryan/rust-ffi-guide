# Using Bindgen For Bindings

A lot of the time you'll have a large amount of existing C (or C++, but more of 
that later) code which you need to integrate your Rust tools into, a great 
example of this is [OpenSSl][openssl] or [git][git]. Unless the C code is 
fairly small or trivial it's not going to be feasible to port to Rust, plus
why re-invent the wheel when you could be doing more interesting stuff?

Luckily with Rust you can have your cake and eat it too. As you'll already know
Rust can transparently call C functions as long as you have declarations for
them so the compiler knows what symbols to use. Enter [bindgen][bindgen].

Bindgen does all the hard work of parsing C-style header files and generating 
the equivalent Rust function definitions. They've also got a 
[great tutorial][tut] for getting started so I'm not going to bother 
reiterating the basics. Instead, we'll try to focus on the more high level 
stuff which comes with using a C library from Rust, seeing as the zero cost
abstractions and high level way of approaching things is probably why you're 
trying to do this FFI stuff in the first place!


## Getting Set Up

My target today will be [libtcod][libtcod], AKA "The Doryen Library". It's a 
C library for creating rogue-like games, so this should be more interesting 
than a basic bindgen hello world.

First we're going to need to get a copy of the source code. You can just go to
bitbucket and download the source as a zip, but I'm going add it as a git 
submodule inside this book's source because that's easier to maintain. 

If you want to do the same then you'll need to install [git-hg][git-hg] then
do something like this:

```bash
$ git-hg clone https://bitbucket.org/libtcod/libtcod ./src/bindgen/libtcod
$ git submodule add ./src/bindgen/libtcod ./src/bindgen/libtcod
$ git commit -am "added libtcod submodule"
```

Now we'll need to compile the source code. Instructions for Ubuntu are 
available [here][ubuntu-instructions]. I've copied across the commands I used
for convenience.

```bash
$ sudo apt-get install curl build-essential make cmake autoconf automake \
       libtool mercurial libasound2-dev libpulse-dev libaudio-dev libx11-dev \
       libxext-dev libxrandr-dev libxcursor-dev libxi-dev libxinerama-dev \
       libxxf86vm-dev libxss-dev libgl1-mesa-dev libesd0-dev libdbus-1-dev \
       libudev-dev libgles1-mesa-dev libgles2-mesa-dev libegl1-mesa-dev
$ sudo apt install libsdl2-dev
$ cd libtcod/build/autotools
$ autoreconf -i
$ ./configure CFLAGS='-O2'
$ make
```

That should compile all the source code, if you go back up to the `libtcod` 
root directory and look in the `./src/` directory you'll see a bunch of newly
created object files.

TODO: Finish this article!
* Mention writing the build.rs script
* Create basic wrappers around the game object to make things more idiomatic
* write a basic libtcod game 




[openssl]: https://github.com/sfackler/rust-openssl
[git]: https://github.com/alexcrichton/git2-rs
[bindgen]: https://github.com/servo/rust-bindgen
[tut]: http://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html
[libtcod]: https://bitbucket.org/libtcod/libtcod
[git-hg]: https://github.com/cosmin/git-hg
[ubuntu-instructions]: https://bitbucket.org/libtcod/libtcod/src/f3486b0851a2acf11efbd2df18fc06501012afef/README-linux-SDL2.md?fileviewer=file-view-default
