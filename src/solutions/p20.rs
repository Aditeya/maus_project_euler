use rug::{Complete, Integer};

const N: u32 = 100;

pub fn s_v1() -> u64 {
    let fact = Integer::factorial(N).complete();

    let mut sum: u64 = 0;
    for i in fact.to_string().chars().map(|x| x.to_digit(10).unwrap()) {
        sum += i as u64;
    }
    sum
}
