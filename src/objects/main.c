#include <stdio.h>
#include "my_vec.h"

int main() {
    MyVec* vec;

    vec = my_vec_new();
    if (vec == NULL) {
        printf("Creation failed!\n");
        return 1;
    }

    printf("The initial length is %d\n", my_vec_len(vec));

    int n = 42;
    my_vec_push(vec, n);
    printf("After pushing 42, the length is %d\n", my_vec_len(vec));

    printf("Iterating over the items in my vec:\n");
    int* numbers = my_vec_contents(vec);

    for(int i = 0; i < my_vec_len(vec); i++) {
        printf("my_vec[%d] = %d\n", i, numbers[i]);
    }

    my_vec_destroy(vec);
    return 0;
}
