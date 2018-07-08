#ifndef PLUGIN_H
#define PLUGIN_H

typedef void (*PluginCallback)(void* data);
typedef const char* (*PluginName)(void* data);
typedef void (*PluginFileSave)(void* data, const char* filename, const char* contents);

typedef struct Plugin {
    // A pointer to an object encapsulating any state this plugin may have.
    void* data;
    // Callback fired immediately after a plugin is loaded. This allows the
    // plugin to do any necessary initialization.
    PluginCallback on_plugin_load;
    // Callback fired immediately before the plugin library is unloaded from
    // memory, allowing to do finalization and clean up any necessary data.
    PluginCallback on_plugin_unload;
    // Callback fired just before a file is saved to disk.
    PluginFileSave on_file_save;
    // Get a pointer to the plugin's name (mainly for debugging purposes).
    PluginName name;
} Plugin;

// The signature of a plugin registration function (with the symbol name,
// `plugin_register`).
typedef Plugin (*PluginRegister)();

#endif // PLUGIN_H
