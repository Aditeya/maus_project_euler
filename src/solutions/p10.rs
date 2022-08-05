use maus_project_euler::prime_sieve;

pub fn s_v1(n: &u32) -> u64 {
    let primes = prime_sieve(n);

    let mut sum: u64 = 0;
    for i in primes.iter() {
        sum += *i as u64;
    }

    sum
}
