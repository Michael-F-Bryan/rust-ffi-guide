import itertools
from main import Primes, Sieve


def test_sieve_limit():
    limit = 10000
    with Sieve(limit) as s:
        assert s.upper_bound() >= limit


def test_zero_is_not_in_prime_list():
    with Primes() as p:
        n = 20
        assert 0 not in list(itertools.islice(p, n))
