use std::env;

fn to_bits(i: u64) -> String {
    //needs zero padding
    format!{"{:064b}", i}.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let map: &str = "----------------------------------------------------------------\n6666555555555544444444443333333333222222222211111111110000000000\n3210987654321098765432109876543210987654321098765432109876543210";

    match args.len() {
        1 => {
            println!("At least one hex value argument is needed.");
        }
        _ => {
            for i in 1..args.len() {
                let trimmed_string: &str = args[i].as_str().trim_start_matches("0x");
                let number = u64::from_str_radix(trimmed_string, 16);
                // to error checking on number here.
                match number {
                    Err(why) => println!("Input error value was not a hex string. {}.", why),
                    Ok(value) => println!("{}\n{}\n", to_bits(value), map),
                }
            }
        }
    }
}
