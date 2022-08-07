use maus_project_euler::sum_of_natural_numbers;

pub fn s_v1(n: &u32) -> u32 {
    sum_of_natural_numbers(n).pow(2) - sum_of_squares(n)
}

fn sum_of_squares(n: &u32) -> u32 {
    n * (n+1) * (2*n + 1) / 6
}
