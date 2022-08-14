use rug::{Complete, Integer};

const N: u32 = 1000;

pub fn s_v1() -> u64 {
    let big_num = Integer::u_pow_u(2, N).complete().to_string();
    let big_num_digits = big_num.chars().map(|x| x.to_digit(10).unwrap() as u8);
    let mut sum: u64 = 0;
    for i in big_num_digits {
        sum += i as u64;
    }

    sum
}
