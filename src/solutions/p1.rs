const N: u64 = 1000;

pub fn s_v1() -> u64 {
    let n = N - 1;
    sum_divisible_by_n(&3, &n) + sum_divisible_by_n(&5, &n) - sum_divisible_by_n(&15, &n)
}

fn sum_divisible_by_n(n: &u64, value: &u64) -> u64 {
    let p = value/n;
    n * p * (p+1) / 2
}
