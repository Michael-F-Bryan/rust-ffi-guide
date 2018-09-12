#include <stdio.h>
#include <assert.h>
#include "tomlreader.h"

void print_error() {
    Error err = last_error();

    fprintf(stderr,
            "Error: %s [%d - %s]\n",
            err.msg,
            err.category,
            category_name(err.category));
}

int main(int argc, char **argv) {
    char *filename = "tomlreader/Cargo.toml";

    if (argc > 1) {
        filename = argv[1];
    }

    printf("Reading %s\n", filename);

    const Value *toml = parse_file(filename);

    if (toml == NULL) {
        print_error();
        return 1;
    }

    // Don't forget to free the value
    value_destroy(toml);
    return 0;
}
