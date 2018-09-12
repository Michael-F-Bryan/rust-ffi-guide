#include <stdio.h>
#include <assert.h>
#include "tomlreader.h"

void print_error(const char*);
char* package_name(const Value *root);

int main(int argc, char **argv) {
    char *filename = "tomlreader/Cargo.toml";

    if (argc > 1) {
        filename = argv[1];
    }

    printf("Reading %s\n", filename);

    const Value *toml = parse_file(filename);

    if (toml == NULL) {
        print_error("Unable to load the file");
        return 1;
    }

    // Try to read out the package name
    char *name = package_name(toml);
    if (name == NULL) {
        print_error("Couldn't find the package name");
    } else {
        printf("Package: %s\n", name);
        free(name);
    }

    // Don't forget to free the value
    value_destroy(toml);
    return 0;
}

// Copy the package name into our own buffer.
char* package_name(const Value *root_toml) {
    const Value *package = value_get(root_toml, "package");

    if (package == NULL) {
        return NULL;
    }

    const Value *name = value_get(package, "name");
    if (name == NULL) {
        return NULL;
    }

    int bytes_required = value_as_str(name, NULL, 0);
    if (bytes_required == 0) {
        return NULL;
    }

    char *buffer = malloc(bytes_required);
    int bytes_written = value_as_str(name, buffer, bytes_required);

    if (bytes_written != bytes_required) {
        free(buffer);
        return NULL;
    } else {
        return buffer;
    }
}

void print_error(const char *msg) {
    Error err = last_error();

    fprintf(stderr,
            "%s: %s [%d - %s]\n",
            msg,
            err.msg,
            err.category,
            category_name(err.category));
}
