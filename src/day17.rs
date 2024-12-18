pub fn solve_part1(input: &str) -> String {
    let (r, p) = parse_puzzle(input);
    let mut execution = Execution::new(&p, r);
    execution.run();
    execution
        .debugger
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn solve_part2(input: &str) -> usize {
    let (r, p) = parse_puzzle(input);
    part2_searcher(&r, &p, 0).unwrap()
}

fn part2_searcher(r: &Registers, p: &Program, a: u64) -> Option<usize> {
    for i in 0..8 {
        let mut execution = Execution::new(p, r.clone());
        let a = a + i;
        execution.registers.a = a;
        execution.run();
        if execution.debugger_matches_program() {
            return Some(a as usize);
        }
        if execution.debugger_matches_program_suffix() {
            // println!("Found {a} = {a:b} debugger = {:?}", &execution.debugger);
            if a == 0 {
                continue;
            }
            if let Some(answer) = part2_searcher(r, p, a << 3) {
                return Some(answer);
            }
        }
    }
    None
}

fn parse_puzzle(input: &str) -> (Registers, Program) {
    let (r, p) = input.split_once("\n\n").unwrap();
    (Registers::parse(r), Program::parse(p))
}

#[derive(Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn new(code: u8) -> Self {
        match code {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("unknown op code {code}"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    operand: u8,
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Self {
        Self {
            opcode: OpCode::new(opcode),
            operand,
        }
    }
}

struct Program {
    instructions: Vec<u8>,
}

impl Program {
    fn parse(input: &str) -> Self {
        Self {
            instructions: input
                .trim()
                .split_once(": ")
                .unwrap()
                .1
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    fn get_instruction(&self, head: usize) -> Instruction {
        Instruction::new(self.instructions[head], self.instructions[head + 1])
    }
}

#[derive(Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn parse(input: &str) -> Self {
        let mut l = input.lines();
        Self {
            a: l.next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
            b: l.next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
            c: l.next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
        }
    }
}

struct Execution<'a> {
    program: &'a Program,
    registers: Registers,
    head: usize,
    debugger: Vec<u8>,
}

impl<'a> Execution<'a> {
    fn new(program: &'a Program, registers: Registers) -> Self {
        Self {
            program,
            registers,
            head: 0,
            debugger: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.head < self.program.instructions.len() - 1 {
            self.step();
        }
    }

    fn step(&mut self) {
        let instr = self.program.get_instruction(self.head);
        // println!("Step {instr:?}");
        match &instr.opcode {
            OpCode::Adv => {
                self.registers.a = self.instr_xdv(instr.operand);
                self.head += 2;
            }
            OpCode::Bxl => {
                self.registers.b ^= self.operand_literal(instr.operand);
                self.head += 2;
            }
            OpCode::Bst => {
                self.registers.b = self.operand_combo(instr.operand) % 8;
                self.head += 2;
            }
            OpCode::Jnz => {
                if self.registers.a != 0 {
                    self.head = self.operand_literal(instr.operand) as usize;
                } else {
                    self.head += 2;
                }
            }
            OpCode::Bxc => {
                self.registers.b ^= self.registers.c;
                self.head += 2;
            }
            OpCode::Out => {
                self.debugger
                    .push((self.operand_combo(instr.operand) % 8) as u8);
                self.head += 2;
            }
            OpCode::Bdv => {
                self.registers.b = self.instr_xdv(instr.operand);
                self.head += 2;
            }
            OpCode::Cdv => {
                self.registers.c = self.instr_xdv(instr.operand);
                self.head += 2;
            }
        }
    }

    fn instr_xdv(&self, operand: u8) -> u64 {
        let num = self.registers.a;
        let p = self.operand_combo(operand);
        // dbg!(p);
        assert!(p < 64);
        num / 2u64.pow(p as u32)
    }

    fn operand_combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => {
                // println!("A");
                self.registers.a
            }
            5 => self.registers.b,
            6 => self.registers.c,
            7 => panic!("reserved operand 7"),
            _ => panic!("operand out of range {operand}"),
        }
    }

    fn operand_literal(&self, operand: u8) -> u64 {
        match operand {
            0..=7 => operand as u64,
            _ => panic!("operand out of range {operand}"),
        }
    }

    fn debugger_matches_program(&self) -> bool {
        self.debugger == self.program.instructions
    }

    fn debugger_matches_program_suffix(&self) -> bool {
        self.debugger[..]
            == self.program.instructions[self.program.instructions.len() - self.debugger.len()..]
    }
}

#[cfg(test)]
const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn example1() {
    let program = Program {
        instructions: vec![2, 6],
    };
    let mut execution = Execution::new(&program, Registers { a: 0, b: 0, c: 9 });
    execution.run();
    assert_eq!(execution.registers.b, 1);
}

#[test]
fn example2() {
    let program = Program {
        instructions: vec![5, 0, 5, 1, 5, 4],
    };
    let mut execution = Execution::new(&program, Registers { a: 10, b: 0, c: 0 });
    execution.run();
    assert_eq!(execution.debugger, vec![0, 1, 2]);
}

#[test]
fn practice_part2() {
    assert_eq!(
        solve_part2(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
        ),
        117440
    );
}
