#ifndef PLUGIN_MANAGER_H
#define PLUGIN_MANAGER_H

#include <vector>
#include <string>
#include <dlfcn.h>
#include "plugin.h"

class PluginManager {
public:
    void load(std::string library);
    void clear();
    void on_file_save(std::string& filename, std::string& contents);
    ~PluginManager();

private:
    std::vector<void*> libraries;
    std::vector<Plugin> plugins;
};

#endif // PLUGIN_MANAGER_H
