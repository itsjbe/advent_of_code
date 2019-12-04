use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let file = File::open("input")?;
    let result = read_input(file).map(get_fuel_partial).sum::<u64>();
    println!("Total fuel required: {}", result);

    Ok(())
}


fn read_input<T, K>(input: T) -> impl Iterator<Item=K>
    where <K as std::str::FromStr>::Err: std::fmt::Display,
          K: Display + FromStr,
          T: Read
{
    BufReader::new(input)
        .lines()
        .filter_map(|line| {
            match line {
                Ok(line) => {
                    println!("  Parsing: {}", line);
                    match K::from_str(&line) {
                        Ok(val) => {
                            Some(val)
                        }
                        Err(err) => {
                            println!("Error parsing number: {}", err);
                            None
                        }
                    }
                }
                Err(err) => {
                    println!("Error reading line: {}", err);
                    None
                }
            }
        })
}

fn get_fuel(mass: u64) -> u64 {
    let mut result = mass as f64 / 3 as f64;
    result = result.floor() - 2.0;
    if result < 0.0 {
        return 0;
    }
    result as u64
}

fn get_fuel_partial(mut mass: u64) -> u64 {
    let mut count = 0;

    loop {
        let mass_tmp = get_fuel(mass) as f64;
        if mass_tmp <= 0.0 {
            break;
        } else {
            count += mass_tmp as u64;
            let mut tmp = get_fuel(mass) as f64 / 3.0 - 2.0;
            tmp = tmp.floor();
            if tmp <= 0.0 {
                break;
            }
            count += tmp as u64;
            mass = tmp as u64;
        }
    }

    return count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = 14;
        let expected = 2;
        let actual = get_fuel(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test2() {
        let input = 1969;
        let expected = 966;
        let actual = get_fuel_partial(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test3() {
        let input = 100756;
        let expected = 50346;
        let actual = get_fuel_partial(input);

        assert_eq!(actual, expected);
    }
}