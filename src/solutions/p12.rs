use std::collections::HashMap;
use maus_project_euler::sum_of_natural_numbers;
use maus_project_euler::prime_sieve_vec;

#[allow(dead_code)]
pub fn s_v1(n: &u64) -> u64 {
    let mut i = 1;
    loop {
        let triangle_num = sum_of_natural_numbers(&i);
        if divisor_length(&triangle_num) >= *n {
            return triangle_num;
        }
        i += 1;
    }
}

pub fn s_v2(limit: &u64) -> u64 {
    let mut n   : u64 = 3;
    let mut d_n : u64 = 2;
    let mut cnt : u64 = 0;
    let (mut n1, mut d_n1, mut exponent): (u64, u64, u64);
    let primes = prime_sieve_vec(&65500);

    while cnt <= *limit {
        n += 1;
        n1 = n;
        if n % 2 == 0 { n1 /= 2; }
        d_n1 = 1;

        for p in primes.iter() {
            if p * p > n1 {
                d_n1 *= 2;
                break;
            }

            exponent = 1;
            while n1 % p == 0 {
                exponent += 1;
                n1 /= p;
            }

            if exponent > 1 { d_n1 *= exponent; }
            if n1 == 1 { break; }
        }

        cnt = d_n * d_n1;
        d_n = d_n1
    }

    // NOTE: n-1 so it isn't sum of natural numbers
    n * (n-1) / 2
}

fn divisor_length(n: &u64) -> u64 {
    let prime_factors = prime_factors(*n);

    let mut count = 1;
    for i in prime_factors.values().map(|x| x+1) {
        count *= i;
    }

    count
}

fn prime_factors(mut n: u64) -> HashMap<u64, u64> {
    let mut prime_factors: HashMap<u64, u64> = HashMap::new();

    let mut count = 0;
    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }
    prime_factors.insert(2, count);

    for i in (3..((n as f64).sqrt() as u64) ).step_by(2) {
        count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        prime_factors.insert(i, count);
    }

    if n > 2 {
        prime_factors.insert(n, 1);
    }

    prime_factors
}
