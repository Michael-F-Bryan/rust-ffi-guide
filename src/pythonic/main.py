import itertools
from cffi import FFI

ffi = FFI()

ffi.cdef("""
        void* sieve_new(unsigned int limit);
        void sieve_destroy(void *sieve);
        unsigned int sieve_upper_bound(void *sieve);
        unsigned int sieve_is_prime(void *sieve, unsigned int n);

        void* primes_new();
        void primes_destroy(void *primes);
        unsigned int primes_next(void *primes);
        """)

primal = ffi.dlopen('./primes/target/debug/libprimes.so')


class Sieve:
    def __init__(self, limit=1000000):
        self.limit = limit
        self.sieve = None

    def __enter__(self):
        self.sieve = primal.sieve_new(self.limit)
        return self

    def __exit__(self, *args):
        primal.sieve_destroy(self.sieve)

    def is_prime(self, n):
        return primal.sieve_is_prime(self.sieve, n) != 0

    def upper_bound(self):
        return primal.sieve_upper_bound(self.sieve)


class Primes:
    def __enter__(self):
        self.iterator = primal.primes_new()
        return self

    def __exit__(self, *args):
        primal.primes_destroy(self.iterator)

    def next(self):
        return primal.primes_next(self.iterator)

    def __iter__(self):
        running = True
        while running:
            prime = self.next()
            yield prime
            running = prime != 0


if __name__ == "__main__":
    with Sieve(10000) as s:
        print(s.is_prime(5))

    with Primes() as p:
        first_20 = list(itertools.islice(p, 20))
        print(first_20)
