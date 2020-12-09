use std::fs;
use std::str::FromStr;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Clone)]
struct Instruction {
    opcode: String,
    value: usize,
    sign: bool,
}

struct Computer {
    program: Vec<Instruction>,
    pc: usize, // Just so we can start @ 0
    acc: i32,
}

struct ComputerState {
    instruction: Instruction,
    pc: usize, // Just so we can start @ 0
    acc: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let opcode = tokens.next().unwrap().to_string();
        let value_tokens = tokens.next().unwrap().split_at(1);
        let sign = match value_tokens.0{
            "+" => true,
            "-" => false,
            _ => {return Err("sign".to_string())}
        };
        let value = value_tokens.1.parse().unwrap();
        
        Ok(Instruction {opcode, value, sign})
    }
}

impl Computer {
    fn new(program: Vec<Instruction>) -> Self {
        Computer {
            program: program.clone(),
            pc: 0, // So the first run will initialize it to zero
            acc: 0,
        }
    }
}

fn operate(sign: bool, a: usize, b: usize) -> usize {
    match sign {
        true => a + b,
        false => a - b,
    }
}

fn signed(sign: bool, a: usize) -> i32 {
    let maybe_neg = a.try_into().unwrap();
    match sign {
        true => maybe_neg,
        false => maybe_neg * -1,
    }
}


impl Iterator for Computer {
    type Item = ComputerState;

    fn next(&mut self) -> Option<Self::Item> {
        let ins = &self.program[self.pc];

        let result = ComputerState {
            instruction: ins.clone(),
            pc: self.pc,
            acc: self.acc,
        };

        match ins.opcode.as_str() {
            "nop" => {self.pc += 1;},
            "acc" => {
                self.acc += signed(ins.sign, ins.value);
                self.pc += 1;
            },
            "jmp" => {
                self.pc = operate(ins.sign, self.pc, ins.value);
            },
            i => {panic!("Unrecognized opcode: {}", i);}
        }

        Some(result)
    }
}

// Composition FTW
struct HaltingComputer(Computer, HashSet<usize>);

impl HaltingComputer {
    fn new(program: Vec<Instruction>) -> Self {
        HaltingComputer(
            Computer::new(program),
            HashSet::new(),
        )
    }
}

impl Iterator for HaltingComputer {
    type Item = ComputerState;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next().unwrap();

        match self.1.insert(next.pc) {
            true => Some(next),
            false => None 
        }
    }
}

// COMPOSITION FTW
struct EscapingComputer(HaltingComputer, usize);

impl EscapingComputer {
    fn new(program: Vec<Instruction>) -> Self {
        let mut result = EscapingComputer(HaltingComputer::new(program), 0);
        result.swap();
        result
    }

    fn swap(&mut self) {
        let mut pgm = self.0.0.program.clone();

        loop {
            match pgm[self.1].opcode.as_str() {
                "nop" => {
                    pgm[self.1].opcode = "jmp".to_string();
                    break;
                },
                "jmp" => {
                    pgm[self.1].opcode = "nop".to_string();
                    break;
                },
                _ => {self.1 += 1;}
            }
        }

            self.1 += 1;
            self.0 = HaltingComputer::new(pgm);
    }

    fn deswap_and_swap(&mut self) {
        self.1 -= 1;
        self.swap();
        self.swap();
    }
}

impl Iterator for EscapingComputer {
    type Item = ComputerState;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => {
                self.deswap_and_swap();
                self.0.next()
            },
            Some(i) => {
                if self.0.0.pc == self.0.0.program.len() {return None}
                Some(i)
            }
        }
    }
}



fn main() {
    let pgm_string = fs::read_to_string("in/08.in").unwrap();
    let program: Vec<Instruction> = pgm_string.lines().map(|s| s.parse::<Instruction>().unwrap()).collect();

    let h_computer = HaltingComputer::new(program.clone());
    println!("{}", h_computer.last().unwrap().acc);

    let e_computer = EscapingComputer::new(program.clone());
    println!("{}", e_computer.last().unwrap().acc);
}
