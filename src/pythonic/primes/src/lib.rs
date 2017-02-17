extern crate primal;
extern crate libc;

use primal::{Primes, Sieve};

#[no_mangle]
pub extern "C" fn sieve_new(limit: libc::c_uint) -> *mut Sieve {
    let s = Sieve::new(limit as usize);
    Box::into_raw(Box::new(s))
}

#[no_mangle]
pub unsafe extern "C" fn sieve_destroy(sieve: *mut Sieve) {
    if !sieve.is_null() {
        Box::from_raw(sieve);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sieve_upper_bound(sieve: *const Sieve) -> libc::c_uint {
    (&*sieve).upper_bound() as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn sieve_is_prime(sieve: *const Sieve, n: libc::c_uint) -> libc::int8_t {
    (&*sieve).is_prime(n as usize) as libc::int8_t
}


#[no_mangle]
pub extern "C" fn primes_new() -> *mut Primes {
    let iterator = Primes::all();
    Box::into_raw(Box::new(iterator))
}

#[no_mangle]
pub unsafe extern "C" fn primes_destroy(primes: *mut Primes) {
    if !primes.is_null() {
        Box::from_raw(primes);
    }
}

/// Get the next prime in the series.
///
/// # Remarks
///
/// If zero is returned then there the iterator is finished.
#[no_mangle]
pub unsafe extern "C" fn primes_next(primes: *mut Primes) -> libc::c_uint {
    match (&mut *primes).next() {
        Some(p) => p as libc::c_uint, 
        None => 0,
    }
}
