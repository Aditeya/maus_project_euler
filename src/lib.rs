use std::collections::HashSet;

pub fn is_prime(n: &u32) -> bool {
    if *n == 1 { return false }
    else if *n < 4 { return true }
    else if n % 2 == 0 { return false }
    else if *n < 9 { return true }
    else if n % 3 == 0 { return false }

    let r = (*n as f64).sqrt().floor() as u32;
    for f in (5..=r).step_by(6) {
        if n % f == 0 { return false }
        if n % (f+2) == 0 { return false }
    }

    true
}

// TODO: find optimisations for prime_sieve
pub fn prime_sieve(n: &u32) -> HashSet<u32> {
    let mut primes: HashSet<u32> = HashSet::from_iter(2..=*n);

    for i in 2..=((*n as f64).sqrt() as usize) {
        if primes.contains(&(i as u32)) && is_prime(&(i as u32)) {
            for j in ((i+i)..=(*n as usize)).step_by(i as usize) {
                primes.remove(&(j as u32));
            }
        }
    }

    primes
}
