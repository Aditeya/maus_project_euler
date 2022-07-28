pub fn s_v1(n: &u32) -> u32 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;

    while b < *n {
        sum += c;

        a = b + c;
        b = c + a;
        c = a + b;
    }

    sum
}
