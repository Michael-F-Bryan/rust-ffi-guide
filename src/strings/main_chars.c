#include <stdio.h>

int count_characters(char *s);

int main() {
    char *s = "hello world!";
    int num_chars = count_characters(s);

    printf("There are %d characters in \"%s\"\n", num_chars, s);
}
