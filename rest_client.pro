CONFIG += debug_and_release
TEMPLATE = app
TARGET = rest_client
INCLUDEPATH += .

DEFINES += QT_DEPRECATED_WARNINGS

# Input
INCLUDEPATH += gui
SOURCES += gui/main.cpp gui/main_window.cpp
HEADERS += gui/main_window.hpp

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