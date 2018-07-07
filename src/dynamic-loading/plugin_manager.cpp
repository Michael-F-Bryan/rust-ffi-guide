#include <dlfcn.h>
#include <iostream>
#include <string>
#include "plugin_manager.h"

void PluginManager::clear() {
    // let each plugin clean up after itself
    for(auto& plugin: plugins) {
        plugin.on_plugin_unload(plugin.data);
    }

    // then unload the DLLs from memory
    for(auto library: libraries) {
        dlclose(library);
    }

    // All plugins and libraries are logically uninitialized, so we need to
    // forget about them.
    plugins.clear();
    libraries.clear();
}

PluginManager::~PluginManager() {
    clear();
}

void PluginManager::load(std::string filename) {
    // Load the library into memory and get a handle to it
    auto library = dlopen(filename.c_str(), RTLD_LAZY);
    if (library == nullptr) {
        std::cerr << "Unable to load the " << filename << " library" << std::endl;
        return;
    }

    // we've loaded the library into memory now, add it to our list of 
    // libraries so we can make sure it's cleaned up properly.
    libraries.push_back(library);

    // Extract the register function
    auto register_func = (PluginRegister) dlsym(library, "plugin_register");
    if (register_func == nullptr) {
        std::cerr << "Couldn't find the \"plugin_register\" function" << std::endl;
        return;
    }

    // Construct the plugin and run its on_plugin_load callback
    Plugin plugin = register_func();
    plugin.on_plugin_load(plugin.data);

    plugins.push_back(plugin);
}

void PluginManager::on_file_save(std::string& filename, std::string& contents) {
    for(auto& plugin: plugins) {
        plugin.on_file_save(plugin.data, filename.c_str(), contents.c_str());
    }
}
