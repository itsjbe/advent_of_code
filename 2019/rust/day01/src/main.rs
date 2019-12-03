use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let file = File::open("input")?;
    let result = read_input(file).map(get_fuel).sum::<u32>();
    println!("Total fuel required: {}", result);

    Ok(())
}


fn read_input<T: Read>(input: T) -> impl Iterator<Item=u32> {
    BufReader::new(input)
        .lines()
        .filter_map(|line| {
            match line {
                Ok(line) => {
                    match u32::from_str(&line) {
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

fn get_fuel(mass: u32) -> u32 {
    let result = mass as f64 / 3 as f64;
    (result.floor() - 2 as f64) as u32
}

fn get_fuel_partial(mut mass: u32) -> u32 {
    let mut count = 0;

    loop {
        let mass_tmp = get_fuel(mass) as f64;
        if mass_tmp <= 0.0 {
            break;
        } else {
            count += mass_tmp as u32;
            let tmp = get_fuel(mass) as f64 / 3.0 - 2.0;
            if tmp <= 0.0 {
                break;
            }
            count += tmp as u32;
            mass = tmp as u32;
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