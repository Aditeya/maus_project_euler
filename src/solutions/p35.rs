use maus_project_euler::*;

pub fn s_v1(prime_limit: &u32) -> u32 {
    let prime_arr = prime_sieve_hashset(&prime_limit);

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
