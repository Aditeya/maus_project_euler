use maus_project_euler::is_prime;

const N: u64 = 10_001;

pub fn s_v1() -> u64 {
    let mut count = 0;
    let mut i = 2;
    loop {
        if is_prime(&i) {
            count += 1;
            if count == N {
                return i
            }
        }
        i += 1;
    }
}
