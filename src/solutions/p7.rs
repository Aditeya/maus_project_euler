use maus_project_euler::is_prime;

pub fn s_v1(n: &u32) -> u32 {
    let mut count = 0;
    let mut i = 2;
    loop {
        if is_prime(&i) {
            count += 1;
            if count == *n {
                return i
            }
        }
        i += 1;
    }
}
