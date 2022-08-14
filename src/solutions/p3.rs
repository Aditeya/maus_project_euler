const N: u64 = 600851475143;

pub fn s_v1() -> u64 {
    let mut num = N.clone();
    let mut last_factor;

    if N % 2 == 0 {
        last_factor = 2;
        num /= 2;
        while num % 2 == 0 {
            num /= 2;
        }
    } else {
        last_factor = 1;
    }

    let mut factor = 3;
    let mut max_factor = (num as f64).sqrt() as u64;

    while num > 1 && factor <= max_factor {
        if num % factor == 0 {
            num /= factor;
            last_factor = factor;
            while num % factor == 0 {
                num /= factor;
            }
            max_factor = (num as f64).sqrt() as u64;
        }

        factor += 2;
    }

    if num == 1 { last_factor }  else { num }
}
