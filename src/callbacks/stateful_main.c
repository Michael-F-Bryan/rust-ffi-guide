#include <stdio.h>

typedef void (*Progress)(void *data, int value);
extern void generate_numbers(int upper, Progress progress, void *data);

typedef struct Statistics {
    int count;
    float average;
} Statistics;

void increment_statistics(void *data, int value) {
    Statistics *stats = (Statistics*)data;
    printf("received %d\n", value);

    float total = stats->average * stats->count;
    total += value;

    stats->count += 1;
    stats->average = total / stats->count;
}

int main() {
    int upper_limit = 10;
    Statistics stats = {0, 0};

    printf("Generating %d numbers\n", upper_limit);
    generate_numbers(upper_limit, increment_statistics, &stats);

    printf("Statistics:\n");
    printf("\tCount: %d\n", stats.count);
    printf("\tAverage: %g\n", stats.average);

    return 0;
}


