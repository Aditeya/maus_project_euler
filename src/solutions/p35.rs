use std::collections::HashSet;

pub fn s_v1(prime_limit: &u32) -> u32 {
    let prime_arr = prime_sieve(&prime_limit);

    let mut count = 0;
    for i in prime_arr.iter() {
        let len = (i.to_string().len()) as u32;
        let mut rot_num = i.clone();

        let mut is_circular_prime = true;
        for _ in 1..len {
            rot_num = rotate_number(&rot_num, &len);
            if !prime_arr.contains(&rot_num) {
                is_circular_prime = false;
                break;
            }
        }

        if is_circular_prime {
            count += 1;
        }
    }

    count
}

fn rotate_number(n: &u32, digits: &u32) -> u32 {
    (n % 10) * (10 as u32).pow(digits - 1) + (n / 10)
}

fn prime_sieve(n: &u32) -> HashSet<u32> {
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

fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false
    }

    for i in 2..=((*n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false
        }
    }
    true
}
