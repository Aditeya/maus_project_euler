pub fn s_v1() -> u64 {
    let long_str = include_str!("../../assets/p022_names.txt");
    let long_str = &long_str[1..(long_str.len() -2)];
    let mut long_str: Vec<&str> = long_str.split("\",\"").collect();

    let mut sum = 0;
    long_str.sort();
    for (i, word) in long_str.iter().enumerate() {
        sum +=  word_sum(word) as u64 * i as u64;
    }

    sum
}

fn word_sum(word: &str) -> u8 {
    word.chars()
        .map(|c| c as u8)
        .map(|c| c - b'A')
        .sum()
}
