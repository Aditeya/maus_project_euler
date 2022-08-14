const N: u64 = 4_000_000;

pub fn s_v1() -> u64 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;

    while b < N {
        sum += c;

        a = b + c;
        b = c + a;
        c = a + b;
    }

    sum
}
