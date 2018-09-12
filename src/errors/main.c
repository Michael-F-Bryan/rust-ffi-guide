#include <stdio.h>
#include "tomlreader.h"

int main(int argc, char **argv) {
    char *filename = "tomlreader/Cargo.toml";

    if (argc > 1) {
        filename = argv[1];
    }

    printf("Reading %s\n", filename);

    return 0;
}
