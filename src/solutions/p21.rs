const N: u64 = 9999;

pub fn s_v1() -> u64 {
    let mut sum = 0;
    for a in 2..=N {
        let b = sum_of_proper_divisors(&a);
        if b > a {
            if sum_of_proper_divisors(&b) == a {
                sum += a + b;
            }
        }
    }

    sum
}

fn sum_of_proper_divisors(n: &u64) -> u64 {
    sum_of_divisors(*n) - n
}

fn sum_of_divisors(mut n: u64) -> u64 {
    let mut sum: u64 = 1;
    let mut p: u64 = 2;

    while n > 1 && p*p <= n {
        if n % p == 0 {
            let mut j = p*p;
            n /= p;
            while n % p == 0 {
                j *= p;
                n /= p;
            }

            sum *= j-1;
            sum /= p-1;
        }

        if p == 2 {
            p = 3;
        } else {
            p += 2;
        }
    }

    if n > 1 {
        sum *= n + 1;
    }

    sum
}
