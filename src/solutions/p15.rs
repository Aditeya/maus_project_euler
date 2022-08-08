pub fn s_v1(n: &u64) -> u64 {
    let mut result: u64 = 1;

    for i in 1..=*n  {
        result = result * (*n + i) / i; // NOTE: *= gives wrong answer for some reason
    }

    result
}
