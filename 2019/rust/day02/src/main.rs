use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::FromIterator;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let mut program_input: Vec<u32> = read_program(file)?;
    restore_old_state(&mut program_input);

    let result = program(&program_input);
    println!("{}", result[0]);
    Ok(())
}

fn restore_old_state(input: &mut [u32]) {
    input[1] = 12;
    input[2] = 2;
}

fn program(input: &[u32]) -> Vec<u32> {
    let mut result = Vec::from_iter(input[..].iter().cloned());

    let step = 4;
    input.iter().step_by(step)
        .enumerate()
        .map(|t| {
            let op_idx = t.0 * step;
            let op = Op::from(*t.1 as u8);
            (op, op_idx)
        })
        .take_while(|t| {
            t.0 != Op::End
        })
        .for_each(|t| {
            let op = t.0;
            let lhs_idx = result[t.1 + 1] as usize;
            let rhs_idx = result[t.1 + 2] as usize;
            let t_idx = result[t.1 + 3] as usize;
            let lhs = result[lhs_idx];
            let rhs = result[rhs_idx];

            match op {
                Op::Add => {
                    result[t_idx] = lhs + rhs;
                }
                Op::Multiply => {
                    result[t_idx] = lhs * rhs;
                }
                op => panic!("unexpected op: {:?}", op)
            }
        });

    result
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
enum Op {
    Add,
    Multiply,
    End,
}

impl From<u8> for Op {
    fn from(val: u8) -> Self {
        match val {
            1 => Op::Add,
            2 => Op::Multiply,
            99 => Op::End,
            val => {
                panic!("unknown operation: {}", val)
            }
        }
    }
}

fn read_program<TRead, TOutput>(input: TRead) -> Result<Vec<TOutput>, Box<dyn Error>>
    where TRead: Read,
          TOutput: FromStr + Display + Debug,
          <TOutput as FromStr>::Err: Display
{
    let mut input_string = Default::default();
    BufReader::new(input)
        .read_to_string(&mut input_string)?;

    let result: Vec<TOutput> = input_string.split(',')
        .filter_map(|str| {
            let result = str.parse::<TOutput>();
            match result {
                Ok(value) => {
                    Some(value)
                }
                Err(err) => {
                    println!("Error parsing value {:?}: {}", str, err);
                    None
                }
            }
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_program() {
        let input = String::from("1,2,3,4,5");

        let expected = vec![1, 2, 3, 4, 5];

        let actual: Vec<u64> = read_program(input.as_bytes()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_operation() {
        let input = vec![
            (1, Op::Add),
            (2, Op::Multiply),
            (99, Op::End)
        ];

        for tpl in input {
            let input = tpl.0;
            let expected = tpl.1;

            let actual = Op::from(input as u8);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_program() {
        let input = vec![
            1, 9, 10, 3,
            2, 3, 11, 0, 99,
            30, 40, 50
        ];
        let expected = vec![
            3500, 9, 10, 70,
            2, 3, 11, 0,
            99,
            30, 40, 50
        ];

        assert_eq!(program(&input), expected)
    }
}
