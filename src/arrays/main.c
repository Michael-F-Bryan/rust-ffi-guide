#include <stdio.h>

double average(long *array, int length);

int main() {
    long arr[20];

    for (int i = 0; i < 20; i++) {
        arr[i] = i*i;
    }

    double avg = average(arr, 20);

    printf("The average is %f\n", avg);
}
