use std::env;
use std::time::Instant;
use std::process;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut problem_number: u32 = 0;
    match args.len() {
        2 => {
            match args[1].parse() {
                Ok(n) => problem_number = n,
                _ => help()
            }
        },
        _ => help()
    }

    match solutions::SOLUTION_FUNCTIONS_HASHMAP.get(&problem_number) {
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

fn help() {
    eprintln!("Usage: maus_project_euler <problem_number>");
    process::exit(1);
}
