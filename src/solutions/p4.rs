pub fn s_v1() -> u64 {
    let mut largest_palindrome = 0;

    let mut a = 999;
    while a >= 100 {
        let mut b;
        let db;

        if a % 11 == 0 {
            b = 999;
            db = 1;
        } else {
            b = 990;
            db = 11;
        }

        while b >= a {
            let p = a*b;
            if p <= largest_palindrome { break; }
            if is_palindrome(&p) {
                largest_palindrome = p;
            }

            b -= db;
        }

        a -= 1;
    }

    largest_palindrome
}

fn is_palindrome(n: &u64) -> bool {
    *n == reverse_number(*n)
}

fn reverse_number(mut n: u64) -> u64 {
    let mut reverse = 0;
    while n > 0 {
        reverse = 10*reverse + n % 10;
        n /= 10;
    }

    reverse
}
