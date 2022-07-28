use std::env;
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

    let mut ans = 0;
    let mut is_answered = true;
    match problem_number {
        1 => ans = solutions::p1::s_v1(&1000),
        2 => ans = solutions::p2::s_v1(&4_000_000),
        _ => {
            is_answered = false;
            println!("Problem Not Solved Yet :(");
        }
    }

    if is_answered {
        println!("p{} ans: {}", problem_number, ans);
    }


}

fn help() {
    eprintln!("Usage: maus_project_euler <problem_number>");
    process::exit(1);
}
