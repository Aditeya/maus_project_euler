pub fn s_v1(n: &u64) -> u64 {
    let mut result: u64 = 1;

    // NOTE: formula is || (n+i)/i . answer is written this way to prevent precendence errors
    for i in 1..=*n  {
        result *= *n + i; 
        result /= i;
    }

    result
}
