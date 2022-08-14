use maus_project_euler::sum_of_natural_numbers;

const N: u64 = 10;

pub fn s_v1() -> u64 {
    sum_of_natural_numbers(&N).pow(2) - sum_of_squares(&N)
}

fn sum_of_squares(n: &u64) -> u64 {
    n * (n+1) * (2*n + 1) / 6
}
