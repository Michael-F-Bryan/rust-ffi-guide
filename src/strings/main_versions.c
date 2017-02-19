#include <stdio.h>

char* version();
void string_destroy(char *s);
int version_with_buffer(char *buf, int len);


int main() {
    char *version_1 = version();
    printf("Version from method 1: %s\n", version_1);
    string_destroy(version_1);

    char buffer[10];
    version_with_buffer(buffer, 10);
    printf("Version from method 2: %s\n", buffer);
}
