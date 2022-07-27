pub fn s_v1(n: &u32) -> u32 {
    let n = n - 1;
    sum_divisible_by_n(&3, &n) + sum_divisible_by_n(&5, &n) - sum_divisible_by_n(&15, &n)
}

fn sum_divisible_by_n(n: &u32, value: &u32) -> u32 {
    let p = value/n;
    n * p * (p+1) / 2
}
