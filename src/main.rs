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

    let mut ans: u64 = 0;
    let mut is_answered = true;
    match problem_number {
        1 => ans = solutions::p1::s_v1(&1000) as u64,
        2 => ans = solutions::p2::s_v1(&4_000_000) as u64,
        3 => ans = solutions::p3::s_v1(&600851475143),
        4 => ans = solutions::p4::s_v1() as u64,
        5 => ans = solutions::p5::s_v1() as u64,
        6 => ans = solutions::p6::s_v1(&10) as u64,
        7 => ans = solutions::p7::s_v1(&10_001) as u64,
        8 => ans = solutions::p8::s_v1(&13) as u64,
        9 => ans = solutions::p9::s_v1(&1000) as u64,
        10 => ans = solutions::p10::s_v1(&2_000_000) as u64,
        11 => ans = solutions::p11::s_v1() as u64,
        35 => ans = solutions::p35::s_v1(&1_000_000) as u64,
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
