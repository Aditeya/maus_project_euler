pub fn s_v1(s: &u32) -> u32 {
    let s = s/2;

    let mut m: u32 = 2;
    let mut n: u32;
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
