use advent2020::errors::{ExecutionError, InstructionParseError, TopLevelError};
use std::{collections::HashSet, env};
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Clone)]
struct Machine {
    instructions: Vec<Instruction>,
    accumulator: isize,
    location: isize,
}

#[derive(Clone)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lowered = s.to_string();

        lowered.make_ascii_lowercase();

        let mut items = s.split(' ');
        let instruction = items.next().ok_or(InstructionParseError::EmptyInstruction)?;
        let operand = items.next().ok_or(InstructionParseError::MissingOperand(instruction.to_string()))?;
        let operand_value = isize::from_str(operand)?;

        match instruction {
            "nop" => Ok(Instruction::NOP(operand_value)),
            "acc" => Ok(Instruction::ACC(operand_value)),
            "jmp" => Ok(Instruction::JMP(operand_value)),
            _ => Err(InstructionParseError::UnknownOpcode(instruction.to_string())),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::NOP(s) => write!(f, "NOP {:+}", s),
            Instruction::ACC(s) => write!(f, "ACC {:+}", s),
            Instruction::JMP(s) => write!(f, "JMP {:+}", s),
        }
    }
}

impl FromStr for Machine {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();

        for line in s.lines() {
            let instruction = Instruction::from_str(line)?;
            instructions.push(instruction);
        }

        Ok(Machine {
            instructions,
            accumulator: 0,
            location: 0,
        })
    }
}

impl Machine {
    fn pretty_print(&self) {
        for (idx, instr) in self.instructions.iter().enumerate() {
            let pointer = if (idx as isize) == self.location { "--> " } else { "    " };
            println!("{} {:04}: {}", pointer, idx, instr);
        }
    }

    fn step(&mut self) -> Result<(), ExecutionError> {
        if self.location < 0 || self.location >= (self.instructions.len() as isize) {
            return Err(ExecutionError::NonExistentLocation(self.location));
        }

        match self.instructions[self.location as usize] {
            Instruction::NOP(_) => self.location += 1,
            Instruction::JMP(x) => self.location += x,
            Instruction::ACC(x) => {
                self.location += 1;
                self.accumulator += x;
            }
        }

        Ok(())
    }

    fn terminates(&mut self) -> Result<(bool, isize), ExecutionError> {
        let mut visited_locations = HashSet::new();
        loop {
            let current_location = self.location;
            let current_accumulator = self.accumulator;

            visited_locations.insert(current_location);
            self.step()?;

            if visited_locations.contains(&self.location) {
                return Ok((false, current_accumulator));
            }

            if self.location == (self.instructions.len() as isize) {
                return Ok((true, self.accumulator));
            }
        }
    }

    fn variants(&self) -> VariantGenerator {
        VariantGenerator {
            next_offset: 0,
            base_machine: self.clone(),
        }
    }
}

struct VariantGenerator {
    next_offset: usize,
    base_machine: Machine,
}

impl Iterator for VariantGenerator {
    type Item = Machine;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_offset >= self.base_machine.instructions.len() {
                return None;
            }

            match self.base_machine.instructions[self.next_offset] {
                Instruction::ACC(_) => self.next_offset += 1,
                Instruction::JMP(x) => {
                    let mut retval = self.base_machine.clone();
                    retval.instructions[self.next_offset] = Instruction::NOP(x);
                    self.next_offset += 1;
                    return Some(retval);
                }
                Instruction::NOP(x) => {
                    let mut retval = self.base_machine.clone();
                    retval.instructions[self.next_offset] = Instruction::JMP(x);
                    self.next_offset += 1;
                    return Some(retval);
                }
            }
        }
    }
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().skip(1).next().expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let machine = Machine::from_str(&contents)?;

    machine.pretty_print();

    // this is part 1
    let (terminated, last_accum) = machine.clone().terminates()?;
    if terminated {
        println!("WARNING: Somehow the initial input terminated.");
    }
    println!("Last accumulator before looping forever: {}", last_accum);

    // this is part 2
    for mut variant in machine.variants() {
        if let Ok((true, final_value)) = variant.terminates() {
            println!("\nFound a variant that halts! Its last value is {}", final_value);
            variant.pretty_print();
        }
    }

    Ok(())
}