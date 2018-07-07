#include <iostream>
#include "plugin_manager.h"

int main(int argc, char** argv) {
    if (argc != 2) {
        std::cerr << "USAGE: " << argv[0] << " <plugin.so>" << std::endl;
        return 1;
    }

    std::cout << "Starting Editor" << std::endl;
    PluginManager pm;

    std::cout << "Loading Plugin: " << argv[1] << std::endl;
    pm.load(argv[1]);

    // create an in-memory copy of our file and save it
    std::string filename = "hello_world.txt";
    std::string buffer;
    pm.on_file_save(filename, buffer);

    // then let's add some text
    buffer += "Hello World!";
    pm.on_file_save(filename, buffer);

    // And some more text
    buffer += "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do"
        "eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad"
        "minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip"
        "ex ea commodo consequat. Duis aute irure dolor in reprehenderit in";

    pm.on_file_save(filename, buffer);

    // oops, we didn't like that very much. Clear the buffer
    buffer.clear();
    pm.on_file_save(filename, buffer);

    std::cout << "Exiting Editor" << std::endl;

    return 0;
}
