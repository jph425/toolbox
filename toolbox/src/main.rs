use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("At least one hex value argument is needed.");
        }
        _ => {
            for i in 1..args.len() {
                println!("{}", args[i])
            }
        }
    }
}
