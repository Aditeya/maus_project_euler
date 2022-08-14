const N: u64 = 20;

pub fn s_v1() -> u64 {
    let mut result: u64 = 1;

    // NOTE: formula is || (n+i)/i . answer is written this way to prevent precendence errors
    for i in 1..=N  {
        result *= N + i; 
        result /= i;
    }

    result
}
