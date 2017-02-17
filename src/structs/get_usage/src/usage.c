#include <sys/time.h>
#include <sys/resource.h>

typedef struct stats {
    struct timeval ru_utime;
    long ru_maxrss;
    long ru_isrss;
} stats;

int get_usage_stats(stats *output) {
    struct rusage raw_usage;
    int ret;
    
    ret = getrusage(RUSAGE_SELF, &raw_usage);

    output->ru_utime = raw_usage.ru_utime;
    output->ru_maxrss = raw_usage.ru_maxrss;
    output->ru_isrss = raw_usage.ru_ixrss;

    return ret;
}
