use std::env;
use std::time::Instant;
use std::process;

use tabled::{TableIteratorExt, Tabled, Style};

mod solutions;

#[derive(Tabled)]
struct TableResult {
    problem_number: u32,
    solution: u64,
    time: u128,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => {
            match args[1].as_str() {
                "--all" => run_all(),
                other => {
                    match other.parse() {
                        Ok(n) => run_solution(&n),
                        _ => help()
                    }
                }
            }
        },
        _ => help()
    }

}

fn help() {
    eprintln!("Usage: maus_project_euler <problem_number>");
    process::exit(1);
}

fn run_solution(problem_number: &u32) {
    match solutions::SOLUTION_FUNCTIONS_HASHMAP.get(problem_number) {
        Some(solution_function) => {
            let now = Instant::now();
            let solution = solution_function();
            let time = now.elapsed().as_nanos();

            println!(" problem: {}", problem_number);
            println!("solution: {}", solution);
            println!("    time: {} ns", time);
        },
        None =>  {
            println!("Problem Not Solved Yet :(");
            process::exit(1);
        }
    };
}

fn run_all() {
    let mut results: Vec<TableResult> = Vec::new();
    for (problem_number, solution_function) in solutions::SOLUTION_FUNCTIONS_HASHMAP.entries {
            let now = Instant::now();
            let solution = solution_function();
            let time = now.elapsed().as_nanos();

            results.push(
                TableResult { 
                    problem_number: *problem_number,
                    solution,
                    time,
                }
            );
    }

    results.sort_by(|a,b| a.problem_number.cmp(&b.problem_number));
    let table = results.table()
        .with(Style::modern())
        .to_string();
    println!("{}",table);
}
