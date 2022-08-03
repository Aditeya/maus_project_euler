pub fn is_prime(n: &u32) -> bool {
    if *n == 1 { return false }
    else if *n < 4 { return true }
    else if n % 2 == 0 { return false }
    else if *n < 9 { return true }
    else if n % 3 == 0 { return false }

    let r = (*n as f64).sqrt().floor() as u32;
    for f in (5..=r).step_by(6) {
        if n % f == 0 { return false }
        if n % (f+2) == 0 { return false }
    }

    true
}
