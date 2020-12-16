use anyhow::Result;
use bitvec::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::{io::BufRead, num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FormatError {
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(#[from] ParseIntError),
    #[error("Invalid instruction format")]
    InvalidFormat,
}

fn read_input() -> Vec<String> {
    let file = File::open("./input/8.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader.lines().map(|f| f.unwrap()).collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Nop,
    Accumulate,
    Jump,
}

impl FromStr for Instruction {
    type Err = FormatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Instruction::Accumulate),
            "jmp" => Ok(Instruction::Jump),
            "nop" => Ok(Instruction::Nop),
            _ => Err(FormatError::InvalidOperation(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    instruction: Instruction,
    argument: i32,
    execution_count: i32,
}

impl FromStr for Operation {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let i: Instruction = iter.next().ok_or(FormatError::InvalidFormat)?.parse()?;
        let argument = iter.next().ok_or(FormatError::InvalidFormat)?.parse()?;
        Ok(Operation {
            instruction: i,
            argument,
            execution_count: 0,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum OperationResult {
    Success,
    InfiniteLoop,
}

impl Operation {
    pub fn execute(&mut self, pc: &mut i32, accumulator: &mut i32) -> OperationResult {
        if self.execution_count > 0 {
            OperationResult::InfiniteLoop
        } else {
            match self.instruction {
                Instruction::Jump => *pc += self.argument,
                Instruction::Nop => *pc += 1,
                Instruction::Accumulate => {
                    *accumulator += self.argument;
                    *pc += 1
                }
            };
            self.execution_count += 1;
            OperationResult::Success
        }
    }

    pub fn revert(&mut self) {
        self.instruction = match self.instruction {
            Instruction::Nop => Instruction::Jump,
            Instruction::Jump => Instruction::Nop,
            Instruction::Accumulate => Instruction::Accumulate,
        };
        self.execution_count = 0;
    }
}

pub struct Program {
    operations: Vec<Operation>,
    program_counter: i32,
    accumulator: i32,
    reverted: BitVec<LocalBits, usize>,
    last_revert_pos: usize,
}

impl Program {
    pub fn find_next_instruction(&self) -> Option<(usize, Operation)> {
        self.operations
            .iter()
            .enumerate()
            .rfind(|(ind, op)| {
                op.instruction != Instruction::Accumulate && !self.reverted.get(*ind).unwrap()
            })
            .map(|(a, &b)| (a, b.clone()))
    }

    pub fn run_with_correction(&mut self) -> Result<i32> {
        while self.run() != OperationResult::Success {
            self.revert_ops();
            self.reset();
        }
        Ok(self.accumulator)
    }
    pub fn run_no_correction(&mut self) -> Result<i32> {
        self.run();
        Ok(self.accumulator)
    }

    fn run(&mut self) -> OperationResult {
        loop {
            let res = self
                .operations
                .iter_mut()
                .nth(self.program_counter as usize)
                .unwrap()
                .execute(&mut self.program_counter, &mut self.accumulator);

            if res == OperationResult::InfiniteLoop
                || (self.program_counter as usize) >= self.operations.len()
            {
                //dbg!(res);
                break;
            }
        }
        if (self.program_counter as usize) >= self.operations.len() {
            OperationResult::Success
        } else {
            OperationResult::InfiniteLoop
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.program_counter = 0;
        for op in self.operations.iter_mut() {
            op.execution_count = 0;
        }
    }

    fn revert_ops(&mut self) {
        let ins = self.find_next_instruction().unwrap();
        {
            self.operations.iter_mut().nth(ins.0).unwrap().revert();
        }
        if self.last_revert_pos > 0 {
            self.operations
                .iter_mut()
                .nth(self.last_revert_pos)
                .unwrap()
                .revert();
        }

        self.reverted.set(ins.0, true);
        self.last_revert_pos = ins.0;
    }
}

pub fn part_i() -> Result<i32> {
    let input = read_input();
    let operations: Vec<Operation> = input.iter().map(|s| s.parse().unwrap()).collect();
    let ol = operations.len();
    let mut p = Program {
        operations,
        program_counter: 0,
        accumulator: 0,
        reverted: BitVec::<LocalBits, usize>::repeat(false, ol),
        last_revert_pos: 0,
    };
    p.run_no_correction()
}

pub fn part_ii() -> Result<i32> {
    let input = read_input();
    let operations: Vec<Operation> = input.iter().map(|s| s.parse().unwrap()).collect();
    let ol = operations.len();
    let mut p = Program {
        operations,
        program_counter: 0,
        accumulator: 0,
        reverted: BitVec::<LocalBits, usize>::repeat(false, ol),
        last_revert_pos: 0,
    };
    p.run_with_correction()
}
