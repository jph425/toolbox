use std::env;

fn to_bits(i: u64) -> String {
    //needs zero padding
    format!{"{:064b}", i}.to_string()
}

fn trim<'a>(s: &'a str, p: &'a str) -> &'a str {
    s.trim_start_matches(p)
}

fn hex_atoi(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 16)
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
                let trimmed_string: &str = trim(args[i].as_str(), "0x");
                let number = hex_atoi(trimmed_string);
                // to error checking on number here.
                match number {
                    Err(why) => println!("Input error value was not a hex string. {}.", why),
                    Ok(value) => println!("{}\n{}\n", to_bits(value), map),
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

