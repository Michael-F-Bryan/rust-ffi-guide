#include <stdio.h>

extern int sum(const int* my_array, int length);

int main() {
    int my_array[8] = {1, 2, 3, 4, 5, 6, 7, 8};

    int total = sum(my_array, 8);

    printf("The total is %d\n", total);
    return 0;
}