use colored::Colorize;
use std::collections::HashMap;

const LIMIT: u64 = 1_000_000;

pub fn s_v1() -> u64 {
    println!("{}", "This solution runs faster in release mode".red());
    let mut sequence_length: HashMap<u64,u32> = HashMap::new();
    sequence_length.insert(1, 1);
    let mut longest_n_chain = 1;
    let mut longest_n = 0;

    for n in (LIMIT/2)..LIMIT {
        let mut length = 0;

        match sequence_length.get(&n) {
            Some(s) => {
                length += s;
            },
            None => {
                let mut temp_length = 0;
                let mut c = n;
                while c != 1 {
                    c = collatz_iterate(&c);
                    temp_length += 1;
                    temp_length += match sequence_length.get(&c) {
                        None => 0,
                        Some(s) => {
                            c = 1;
                            *s
                        },
                    };
                }
                sequence_length.insert(n, temp_length);
                length += temp_length;
            }
        };

        if length > longest_n_chain {
            longest_n_chain = length;
            longest_n = n;
        }
    }

    longest_n
}

fn collatz_iterate(n: &u64) -> u64 {
    if *n == 1 { return 1; }

    if n % 2 == 0 {
        n/2
    } else {
        3 * n + 1
    }
}
