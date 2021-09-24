use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} path_to_input.csv", &args[0]);
        process::exit(42);
    } else {
        exercise_sjs::run(&args[1]);
    }
}
