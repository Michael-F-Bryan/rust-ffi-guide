import pytest
import itertools
from main import Primes, Sieve


def test_sieve_limit():
    limit = 10000
    with Sieve(limit) as s:
        assert s.upper_bound() >= limit


def test_checking_above_upper_bound_is_an_error():
    limit = 10
    with Sieve(limit) as s:
        with pytest.raises(IndexError):
            s.is_prime(101)


def test_zero_is_not_in_prime_list():
    with Primes() as p:
        n = 20
        assert 0 not in list(itertools.islice(p, n))


def test_number_primes_asked_is_given():
    with Primes() as p:
        n = 20
        assert len(list(itertools.islice(p, n))) == n
