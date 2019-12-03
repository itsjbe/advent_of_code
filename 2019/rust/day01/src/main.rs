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
