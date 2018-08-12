#include <unistd.h>

typedef void (*Progress)(float);

int long_computation(int n, Progress progress) {
    for(int i = 0; i < n; i++) {
        usleep(50*1000);
        progress((double)i/n * 100.0);
    }

    return n*n;
}
