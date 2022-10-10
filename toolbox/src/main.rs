use colored::*;
use std::env;

fn to_bits(i: u64) -> String {
    //needs zero padding
    format! {"{:064b}", i}
}

fn trim<'a>(s: &'a str, p: &'a str) -> &'a str {
    s.trim_start_matches(p)
}

fn hex_atoi(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 16)
}

fn idx_of_1s(num: u64) -> Vec<u64> {
    let mut r: Vec<u64> = vec![];

    for i in 0..64 {
        let lsb: u64 = (num >> i) & 1;
        if lsb == 1 {
            r.push(i);
        }
    }

    r
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const MAP: &str = "----------------------------------------------------------------\n6666555555555544444444443333333333222222222211111111110000000000\n3210987654321098765432109876543210987654321098765432109876543210";

    match args.len() {
        1 => {
            println!("At least one hex value argument is needed.");
        }
        _ => {
            for arg in args.iter().skip(1) {
                let trimmed_string: &str = trim(arg.as_str(), "0x");
                let number = hex_atoi(trimmed_string);
                // to error checking on number here.
                match number {
                    Err(why) => println!("Input error value was not a hex string. {}.", why),
                    Ok(value) => {
                        let mut ones: Vec<u64> = idx_of_1s(value);
                        //ones.reverse();

                        let mut s: String = String::new();
                        let mut bit: u64 = 65;

                        if !ones.is_empty() {
                            bit = ones.pop().unwrap();
                        }

                        for i in 0..64 {
                            // ones is ordered, so this will only require one pass.
                            let cursor_pos: u64 = 63 - i;
                            if cursor_pos == bit {
                                s.push('^');
                                if ones.is_empty() {
                                    break;
                                } else {
                                    bit = ones.pop().unwrap();
                                }
                            } else {
                                s.push(' ');
                            }
                        }
                        println!("{}", to_bits(value));
                        println!("{}", MAP.blue());
                        println!("{}\n", s.blue().bold());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_nothing() {
        let s = "abcd";
        let p = "";
        let r = trim(s, p);

        assert_eq!(s, r);
    }

    #[test]
    fn trim_0x() {
        let s = "0x1234";
        let p = "0x";
        let r = trim(s, p);

        assert_eq!(r, "1234");
    }

    #[test]
    fn trim_nomatch() {
        let s = "1234";
        let p = "0x";
        let r = trim(s, p);

        assert_eq!(s, r);
    }

    const ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    #[test]
    fn bits_zeros() {
        let s = 0;
        let r = to_bits(s);

        assert_eq!(r, ZEROS.to_owned());
    }

    #[test]
    fn bits_ones() {
        let s = 0xFFFF_FFFF_FFFF_FFFF;
        let r = to_bits(s);

        assert_eq!(r, ONES.to_owned());
    }

    //compile-time failure: good
    // #[test]
    // fn hex_ovfl() {
    //     let s = 1 << 65;
    //     let r = to_bits(s);

    //     assert_eq!(r, ZEROS.to_owned())
    // }

    #[test]
    fn hex_zero() {
        let s = "0";
        let r = hex_atoi(s).unwrap();

        assert_eq!(r, 0x0);
    }

    #[test]
    fn hex_ones() {
        let s = "FFFFFFFFFFFFFFFF";
        let r = hex_atoi(s).unwrap();

        assert_eq!(r, 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn hex_one() {
        let s = "1";
        let r = hex_atoi(s).unwrap();

        assert_eq!(r, 0x1);
    }
}
