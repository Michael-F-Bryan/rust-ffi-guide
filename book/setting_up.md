# Setting Up

Before we can start doing any coding we need to get a build environment set up
and run a hello world program to check everything works.

## Setting up Qt and the Build System

First, create a new `qmake` project in a directory of your choosing.

```bash
$ mkdir rest_client && cd rest_client
$ mkdir gui
$ touch gui/main.cpp
$ qmake -project
```

You'll then want to make sure your `rest_client.pro` file (the file specifying 
the project and build settings) looks something like this.

```
TEMPLATE = app
TARGET = rest_client
INCLUDEPATH += .

DEFINES += QT_DEPRECATED_WARNINGS

# Input
SOURCES += gui/main.cpp

QT += widgets
```

This says we're building an `app` called `rest_client` where the only source 
files (for the moment) are `gui/main.cpp`. We also need to tell Qt to include 
the widget modules with `QT += widgets`.

Our `main.cpp` is still empty, lets rectify that by adding in a [button].


```cpp
#include <QtWidgets/QApplication>
#include <QtWidgets/QPushButton>

int main(int argc, char **argv) {
  QApplication app(argc, argv);

  QPushButton button("Hello World");
  button.show();

  app.exec();
}
```

Now we can compile and run this basic program to make sure everything is 
working. You'll probably want to create a separate `build/` directory so you 
don't pollute the rest of the project with random build artefacts.

```
$ mkdir build && cd build
$ qmake ..
$ make
$ ./rest_client
```


## Building Rust with Qmake

Next we need to create the Rust project.

```
$ cargo new client
```

To make it accessible from C++ we need to make sure `cargo` generates a 
dynamically linked library. This is just a case of tweaking our `Cargo.toml`. 


```toml
# client/Cargo.toml

[package]
name = "client"
version = "0.1.0"
authors = ["Michael Bryan <michaelfbryan@gmail.com>"]
description = "The business logic for a REST client"
repository = "https://github.com/Michael-F-Bryan/rust-ffi-guide"

[dependencies]

[lib]
crate-type = ["cdylib"]
```

If you then compile the project you'll see `cargo` build a shared object 
(`*.so`) instead of the normal `*.rlib` file.

```
$ cargo build
$ ls target/debug/
build  deps  examples  incremental  libclient.d  libclient.so  native
```

Now we know the Rust compiles, we just need to hook it up to `qmake`. To do 
this, we need to add a custom build target to our `rest_client.pro` project 
file.

First add a couple useful variables

```
CONFIG(debug, debug|release) {
    CARGO_TARGET = debug
    CARGO_CMD = cargo build 
} else {
    CARGO_TARGET = release
    CARGO_CMD = cargo build --release
}
```

This will check whether we are running a `debug` or `release` build and set the
`CARGO_CMD` and `CARGO_TARGET` variables accordingly. 

Next we add a new build target, `client`, which will `cd` into our `client/` 
directory, call `cargo` using the `CARGO_CMD` command we defined earlier, then 
copy the compiled library to the output directory. Cargo does its own dependency
checking and will recompile as necessary, so we'll just invoke it on every build
(the `client.depends = FORCE` bit).


```
CLIENT_DIR = $$PWD/client
CLIENT_SO = $$CLIENT_DIR/target/$$CARGO_TARGET/libclient.so

client.target = $$OUT_PWD/libclient.so
client.depends = FORCE
client.commands = cd $$CLIENT_DIR && $$CARGO_CMD && cp $$CLIENT_SO $$client.target
```

And finally we need to let `qmake` know there's an extra target as well as a new
build artefact.

```
QMAKE_EXTRA_TARGETS += client
OBJECTS += $$client.target
```

Your full project file should now look something like this

```
# rest_client.pro

CONFIG += debug
TEMPLATE = app
TARGET = rest_client
INCLUDEPATH += .

DEFINES += QT_DEPRECATED_WARNINGS

# Input
SOURCES += gui/main.cpp

QT += widgets

CONFIG(debug, debug|release) {
    CARGO_TARGET = debug
    CARGO_CMD = cargo build 
} else {
    CARGO_TARGET = release
    CARGO_CMD = cargo build --release
}

CLIENT_DIR = $$PWD/client
CLIENT_SO = $$CLIENT_DIR/target/$$CARGO_TARGET/libclient.so

client.target = $$OUT_PWD/libclient.so
client.depends = FORCE
client.commands = cd $$CLIENT_DIR && $$CARGO_CMD && cp $$CLIENT_SO $$client.target

OBJECTS += $$client.target
QMAKE_EXTRA_TARGETS += client
```


## Calling Rust from C++

So far we've just made sure everything compiles, however the C++ and Rust code 
are still completely independent. The next task is to check the Rust library is
linked to properly by calling a function from C++.

First we add a dummy function to the `lib.rs`.


```rust
#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello World!");
}
```

There's a lot going on here, so lets step through it bit by bit.

The `#[no_mangle]` attribute indicates to the compiler that it shouldn't mangle 
the function's name during compilation. According to Wikipedia, 
[*name mangling*][mangle]:

> In compiler construction, name mangling (also called name decoration) is a
> technique used to solve various problems caused by the need to resolve unique
> names for programming entities in many modern programming languages.
>
> It provides a way of encoding additional information in the name of a
> function, structure, class or another datatype in order to pass more semantic
> information from the compilers to linkers.
>
> The need arises where the language allows different entities to be named
> with the same identifier as long as they occupy a different namespace (where
> a namespace is typically defined by a module, class, or explicit namespace
> directive) or have different signatures (such as function overloading).

**TL:DR;** *it's a way for compilers to generate multiple instances of a function
which accepts different types or parameters. Without it we wouldn't be able to
have things like generics or function overloading without name clashes.*

If this function is going to be called from C++ we need to specify the 
[calling convention] (the `extern "C"` bit). This tells the compiler low level
things like how arguments are passed between functions. By far the most common 
convention is to "just do what C does".

The rest of the function declaration should be fairly intuitive.

After recompiling (`cd build && qmake .. && make`) you can inspect the generated
binary using `nm` to make sure the `hello_world()` function is there.

```
$ nm libclient.so | grep ' T '
0000000000003330 T hello_world
00000000000096c0 T __rdl_alloc
00000000000098d0 T __rdl_alloc_excess
0000000000009840 T __rdl_alloc_zeroed
0000000000009760 T __rdl_dealloc
0000000000009a20 T __rdl_grow_in_place
0000000000009730 T __rdl_oom
0000000000009780 T __rdl_realloc
0000000000009950 T __rdl_realloc_excess
0000000000009a30 T __rdl_shrink_in_place
0000000000009770 T __rdl_usable_size
0000000000015ad0 T rust_eh_personality
```

The `nm` tool lists all the symbols in a binary as well as their addresses 
(the hex bit in the first column) and what type of symbol they are. All 
functions are in the **T**ext section of the binary, so you can use grep to view
only the exported functions.

Now we have a working library, why don't we make the GUI program less like a 
contrived example and more like a real-life application?

The first thing is to pull our main window out into its own header file.

```cpp
// gui/main_window.cpp

#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  explicit MainWindow(QWidget *parent = 0);
private slots:
  void onClick();

private:
  QPushButton *button;
};
```

Here we've declared a `MainWindow` class which contains our trusty `QPushButton`
and has a single constructor and click handler.

We also need to fill out the `MainWindow` methods and hook up the button's 
`released` signal to our `onClick()` click handler.

```cpp
// gui/main_window.cpp

#include "gui/main_window.hpp"

extern "C" {
void hello_world();
}

void MainWindow::onClick() { 
    // Call the `hello_world` function to print a message to stdout
    hello_world(); 
    }

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  button = new QPushButton("Click Me", this);
  button->show();
  
  // Connect the button's `released` signal to `this->onClick()`
  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}
```

Don't forget to update `main.cpp` to use the new `MainWindow`.

```cpp
// gui/main.cpp

#include "gui/main_window.hpp"
#include <QtWidgets/QApplication>

int main(int argc, char **argv) {
  QApplication app(argc, argv);

  MainWindow *ui = new MainWindow();
  ui->show();

  app.exec();
}

```

Finally, `cmake` needs to be told that there are now two more source files to 
keep track of. Update the section underneath the `# Input` comment to include
our `main_window` header and `cpp` files.

```
...
# Input
INCLUDEPATH += gui
SOURCES += gui/main.cpp gui/main_window.cpp
HEADERS += gui/main_window.hpp
...
```

And, now when you compile and run it "Hello World" wil be printed to the console
every time you click on the button. 

If you got to this point then congratulations, you've just finished the most 
difficult part - getting everything to build!


[button]: http://doc.qt.io/qt-5/qpushbutton.html
[mangle]: https://en.wikipedia.org/wiki/Name_mangling
[calling convention]: https://en.wikipedia.org/wiki/Calling_convention