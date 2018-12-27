use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::collections::HashSet;

/*
Addition:
addr (add register) stores into register C the result of adding register A and register B.
addi (add immediate) stores into register C the result of adding register A and value B.

Multiplication:
mulr (multiply register) stores into register C the result of multiplying register A and register B.
muli (multiply immediate) stores into register C the result of multiplying register A and value B.

Bitwise AND:
banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.

Bitwise OR:
borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.

Assignment:
setr (set register) copies the contents of register A into register C. (Input B is ignored.)
seti (set immediate) stores value A into register C. (Input B is ignored.)

Greater-than testing:
gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.

Equality testing:
eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
*/

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
enum OpType {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

fn get_all_opcodes() -> Vec<OpType> {
    vec![OpType::ADDR, OpType::ADDI, OpType::MULR, OpType::MULI, OpType::BANR, OpType::BANI, OpType::BORR, OpType::BORI, OpType::SETR, OpType::SETI, OpType::GTIR, OpType::GTRI, OpType::GTRR, OpType::EQIR, OpType::EQRI, OpType::EQRR]
}

#[derive(Debug, Clone, PartialEq)]
struct Part1TestCase {
    before: Vec<i32>,
    after: Vec<i32>,
    opcode_raw: i32,
    op_rega: i32,
    op_regb: i32,
    op_rego: i32,
}

impl fmt::Display for Part1TestCase {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(
            f,
            "{:?}, {:?} {:?} -> {:?} = {:?} -> {:?}",
            self.opcode_raw,
            self.op_rega,
            self.op_regb,
            self.op_rego,
            self.before,
            self.after
        )
    }
}

fn apply_opcode(op: OpType, rega: i32, regb: i32, rego: i32, reg: &[i32]) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::from(reg);
    out[rego as usize] = match op {
        OpType::ADDR => out[rega as usize] + out[regb as usize],
        OpType::ADDI => out[rega as usize] + regb,
        OpType::MULR => out[rega as usize] * out[regb as usize],
        OpType::MULI => out[rega as usize] * regb,
        OpType::BANR => out[rega as usize] & out[regb as usize],
        OpType::BANI => out[rega as usize] & regb,
        OpType::BORR => out[rega as usize] | out[regb as usize],
        OpType::BORI => out[rega as usize] | regb,
        OpType::SETR => out[rega as usize],
        OpType::SETI => rega,
        OpType::GTIR => (rega > out[regb as usize]) as i32,
        OpType::GTRI => (out[rega as usize] > regb) as i32,
        OpType::GTRR => (out[rega as usize] > out[regb as usize]) as i32,
        OpType::EQIR => (rega == out[regb as usize]) as i32,
        OpType::EQRI => (out[rega as usize] == regb) as i32,
        OpType::EQRR => (out[rega as usize] == out[regb as usize]) as i32,
    };
    out
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> (Vec<Part1TestCase>, Vec<Vec<i32>>) {
    let test_case: Regex =
        Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)]").unwrap();

    let mut part1_ret: Vec<Part1TestCase> = Vec::new();
    let mut end = 0;
    for c in test_case.captures_iter(input) {
        //println!("{} -> {:?}", input, c);
        let nums = c
            .iter()
            .filter_map(|i| match i {
                Some(m) => m.as_str().parse::<i32>().ok(),
                None => None,
            })
            .collect::<Vec<i32>>();
        end = c.get(0).unwrap().end();
        part1_ret.push(Part1TestCase {
            before: nums[0..4].to_vec(),
            after: nums[8..12].to_vec(),
            opcode_raw: nums[4],
            op_rega: nums[5],
            op_regb: nums[6],
            op_rego: nums[7],
        });
    }

    let mut part2_ret: Vec<Vec<i32>> = Vec::new();
    for l in input[end..].lines() {
        if l.is_empty() {
            continue
        }
        part2_ret.push(l.split_whitespace().map(|d| d.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }
    (part1_ret, part2_ret)
}

#[aoc(day16, part1)]
fn solve_part1(input: &(Vec<Part1TestCase>, Vec<Vec<i32>>)) -> i32 {
    let mut total: i32 = 0;
    'test: for test_case in &input.0 {
        let mut count: i32 = 0;
        for op in get_all_opcodes() {
            let out = apply_opcode(op, test_case.op_rega, test_case.op_regb, test_case.op_rego, test_case.before.as_slice());
            if out.iter().zip(test_case.after.iter()).all(|(a, b)| *a == *b) {
                //println!("{:?} -> {:?} {:?} {:?} {:?} -> {:?} <=> {:?}", test_case.before, op, test_case.op_rega, test_case.op_regb, test_case.op_rego, out, test_case.after);
                count += 1;
            }
            if count >= 3 {
                total += 1;
                continue 'test;
            }
        }
    }
    total
}

fn assign_code(constraints: &[HashSet<OpType>], assigned_list: &[Option<OpType>], unassigned_list: &[OpType]) -> Option<Vec<OpType>> {
    //println!("Assigned: {:?}\nUnassigned:{:?}", assigned_list, unassigned_list);
    if unassigned_list.is_empty() {
        return Some(assigned_list.iter().map(|o| o.unwrap()).collect::<Vec<OpType>>());
    }
    let op = unassigned_list[0];
    let mut attempt = assigned_list.to_vec();
    for code in 0..16usize {
        if attempt[code].is_none() && !constraints[code].contains(&op) {
            attempt[code] = Some(op);
            if let Some(assignment) = assign_code(constraints, &attempt.as_slice(), &unassigned_list[1..]) {
                return Some(assignment);
            }
            attempt[code] = None;
        }
    }
    None
}

#[aoc(day16, part2)]
fn solve_part2(input: &(Vec<Part1TestCase>, Vec<Vec<i32>>)) -> i32 {
    // Figure out a mapping from opcode number to operation type
    let mut incompatible_ops: Vec<HashSet<OpType>> = vec![HashSet::new(); 16];

    for test_case in &input.0 {
        for op in get_all_opcodes() {
            let out = apply_opcode(op, test_case.op_rega, test_case.op_regb, test_case.op_rego, test_case.before.as_slice());
            if out.iter().zip(test_case.after.iter()).any(|(a, b)| *a != *b) {
                incompatible_ops[test_case.opcode_raw as usize].insert(op);
            }
        }
    }
    println!("{:#?}", incompatible_ops);

    let code_map = match assign_code(incompatible_ops.as_slice(), &[None; 16], get_all_opcodes().as_slice()) {
        Some(m) => m,
        None => panic!("Unable to build code map"),
    };
    println!("{:#?}", code_map);

    // Run the program
    let mut registers: Vec<i32> = vec![0i32; 4];
    for cmd in &input.1 {
        registers = apply_opcode(code_map[cmd[0] as usize], cmd[1], cmd[2], cmd[3], &registers);
    }
    registers[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input("Before: [0, 1, 2, 3]\n4 5 6 7\nAfter:  [8, 9, 10, 11]\nBefore: [3, 2, 1, 0]\n7 6 5 4\nAfter:  [11, 10, 9, 8]\n").0,
            vec![
            Part1TestCase{
                before: vec![0, 1, 2, 3],
                after: vec![8, 9, 10, 11],
                opcode_raw: 4,
                op_rega: 5,
                op_regb: 6,
                op_rego: 7,
            },
            Part1TestCase{
                before: vec![3, 2, 1, 0],
                after: vec![11, 10, 9, 8],
                opcode_raw: 7,
                op_rega: 6,
                op_regb: 5,
                op_rego: 4,
            }]
        );
    }

    #[test]
    fn test_solve_part1() {
        let test_case: (Vec<Part1TestCase>, Vec<Vec<i32>>) = parse_input("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]");
        assert_eq!(1, solve_part1(&test_case));
    }
}
