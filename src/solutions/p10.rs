use maus_project_euler::prime_sieve_hashset;

const N: u64 = 2_000_000;

pub fn s_v1() -> u64 {
    let primes = prime_sieve_hashset(&N);

    let mut sum: u64 = 0;
    for i in primes.iter() {
        sum += *i as u64;
    }

    sum
}
