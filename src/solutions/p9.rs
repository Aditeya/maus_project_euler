pub fn s_v1(s: &u64) -> u64 {
    let s = s/2;

    let mut m: u64 = 2;
    let mut n: u64;
    'found: loop {
        n = 2;
        while n < m {
            if m*(m+n) == s {
                break 'found;
            }
            n+=1;
        }
        m+=1
    }

    2*n*m.pow(5) - 2*m*n.pow(5)
}
