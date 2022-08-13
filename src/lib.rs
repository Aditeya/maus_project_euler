use std::collections::HashSet;

pub fn sum_of_natural_numbers(n: &u64) -> u64 {
    n * (n + 1) / 2
}

pub fn is_prime(n: &u64) -> bool {
    if *n == 1 { return false }
    else if *n < 4 { return true }
    else if n % 2 == 0 { return false }
    else if *n < 9 { return true }
    else if n % 3 == 0 { return false }

    let r = (*n as f64).sqrt().floor() as u64;
    for f in (5..=r).step_by(6) {
        if n % f == 0 { return false }
        if n % (f+2) == 0 { return false }
    }

    true
}

pub fn prime_sieve_hashset(limit: &u64) -> HashSet<u64> {
    let root_limit = (*limit as f64).sqrt() as usize;
    let mut primes: HashSet<u64> = HashSet::from_iter((3..=*limit).step_by(2));
    primes.insert(2);

    for i in (3..=root_limit).step_by(2) {
        if primes.contains(&(i as u64)) {
            for j in ((i*i)..=(*limit as usize)).step_by(2*i as usize) {
                primes.remove(&(j as u64));
            }
        }
    }

    primes
}

pub fn prime_sieve_vec(limit: &u64) -> Vec<u64> {
    let primes = prime_sieve_hashset(limit);
    let mut primes = primes.into_iter().collect::<Vec<u64>>();
    primes.sort_unstable();

    primes
}
