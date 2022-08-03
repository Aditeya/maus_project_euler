pub fn s_v1() -> u32 {
    let mut i = 2;
    while !is_divisible(&i) {
        i += 1;
    }
    i
}

fn is_divisible(n: &u32) -> bool {
    for i in [20, 19, 18, 17, 16, 15, 14, 13, 11] {
        if n % i != 0 {
            return false
        }
    }
    true
}
