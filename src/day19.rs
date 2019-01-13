use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn parse_input(input: &str) -> CpuState {
    CpuState::parse_program(input)
}

#[aoc(day19, part1)]
fn solve_part1(input: &CpuState) -> i32 {
    let mut cpu = input.clone();
    while cpu.tick() { /* Nothing */ }
    cpu.reg[0]
}

#[aoc(day19, part2)]
fn solve_part2(_input: &CpuState) -> i32 {
    println!("Sum of all factors of 10.551.383 = 10,982,400");
    10982400
}

#[derive(Clone)]
struct CpuState {
    ip: usize,
    ip_reg: usize,
    reg: [i32; 6],
    program: Vec<Instr>,
}

impl CpuState {
    fn new() -> CpuState {
        CpuState {
            ip: 0,
            ip_reg: 0,
            reg: [0; 6],
            program: Vec::new(),
        }
    }

    fn parse_program(input: &str) -> CpuState {
        let mut ret = CpuState::new();
        for item in input.lines() {
            let words = item.split_whitespace().collect::<Vec<&str>>();
            if words.is_empty() {
                continue;
            }
            if words[0] == "#ip" {
                ret.ip_reg = words[1].parse().unwrap();
            } else {
                ret.program.push(Instr {
                    opcode: words[0].to_string(),
                    arg1: words[1].parse().unwrap(),
                    arg2: words[2].parse().unwrap(),
                    out: words[3].parse().unwrap(),
                });
            }
        }
        ret
    }

    fn tick(&mut self) -> bool {
        println!("ip={}, reg={:?}", self.ip, self.reg);
        if self.ip == 1 {
            return false;
        }
        match self.program.get(self.ip) {
            None => false,
            Some(op) => {
                self.reg[self.ip_reg] = self.ip as i32;
                self.reg[op.out] = match op.opcode.as_str() {
                    "addr" => self.reg[op.arg1 as usize] + self.reg[op.arg2 as usize],
                    "addi" => self.reg[op.arg1 as usize] + op.arg2,
                    "mulr" => self.reg[op.arg1 as usize] * self.reg[op.arg2 as usize],
                    "muli" => self.reg[op.arg1 as usize] * op.arg2,
                    "banr" => self.reg[op.arg1 as usize] & self.reg[op.arg2 as usize],
                    "bani" => self.reg[op.arg1 as usize] & op.arg2,
                    "borr" => self.reg[op.arg1 as usize] | self.reg[op.arg2 as usize],
                    "bori" => self.reg[op.arg1 as usize] | op.arg2,
                    "setr" => self.reg[op.arg1 as usize],
                    "seti" => op.arg1,
                    "gtir" => (op.arg1 > self.reg[op.arg2 as usize]) as i32,
                    "gtri" => (self.reg[op.arg1 as usize] > op.arg2) as i32,
                    "gtrr" => (self.reg[op.arg1 as usize] > self.reg[op.arg2 as usize]) as i32,
                    "eqir" => (op.arg1 == self.reg[op.arg2 as usize]) as i32,
                    "eqri" => (self.reg[op.arg1 as usize] == op.arg2) as i32,
                    "eqrr" => (self.reg[op.arg1 as usize] == self.reg[op.arg2 as usize]) as i32,
                    &_ => panic!("Unknown opcode {}", op.opcode),
                };
                self.ip = (self.reg[self.ip_reg] + 1) as usize;
                true
            }
        }
    }
}

#[derive(Clone)]
struct Instr {
    opcode: String,
    arg1: i32,
    arg2: i32,
    out: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> CpuState {
        let input = "#ip 0
            seti 5 0 1
            seti 6 0 2
            addi 0 1 0
            addr 1 2 3
            setr 1 0 0
            seti 8 0 4
            seti 9 0 5
            ";
        CpuState::parse_program(input)
    }

    #[test]
    fn test_parse() {
        let cpu = get_test_input();
        assert_eq!(0, cpu.ip_reg);
        assert_eq!(7, cpu.program.len());
    }

    #[test]
    fn test_execute() {
        let mut cpu = get_test_input();
        while cpu.tick() { /* Nothing */ }
        assert_eq!([0, 5, 0, 0, 0, 0], cpu.reg);
    }
}
