advent_of_code::solution!(2);
use itertools::iproduct;
use nom::{
    IResult, Parser,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
};

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(input)
}
fn parser(s: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(char(','), parse_number).parse(s)
}

fn binop(program: &mut Vec<usize>, index: usize, op: fn(usize, usize) -> usize) {
    let a = program[program[index + 1]];
    let b = program[program[index + 2]];
    let out = program[index + 3];
    program[out] = op(a, b);
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut program) = parser(input).ok()?;
    program[1] = 12;
    program[2] = 2;
    let mut index = 0;
    while let Some(opcode) = program.get(index).copied() {
        match opcode {
            1 => binop(&mut program, index, |a, b| a + b),
            2 => binop(&mut program, index, |a, b| a * b),
            99 => break,
            _ => break,
        }
        index += 4;
    }
    program.get(0).copied().map(|v| v as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, memory) = parser(input).ok()?;
    let range = 0..=99;

    iproduct!(range.clone(), range.clone())
        .find(|(noun, verb)| {
            let mut program = memory.clone();
            program[1] = *noun;
            program[2] = *verb;
            let mut index = 0;
            while let Some(opcode) = program.get(index).copied() {
                match opcode {
                    1 => binop(&mut program, index, |a, b| a + b),
                    2 => binop(&mut program, index, |a, b| a * b),
                    99 => break,
                    _ => break,
                }
                index += 4;
            }
            program.get(0).copied().unwrap_or(0) == 19690720
        })
        .map(|(noun, verb)| (100 * noun + verb) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
