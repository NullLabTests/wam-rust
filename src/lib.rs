use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WamError {
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Unification failed")]
    UnificationError,
    #[error("Unknown variable: {0}")]
    UnknownVariable(String),
}

/// Term represents a Prolog term in the WAM
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Variable(String),
    Constant(String),
    Structure {
        functor: String,
        args: Vec<Term>,
    },
}

/// Register types in WAM
#[derive(Debug, Clone, Copy)]
pub enum Register {
    Permanent(usize),
    Temporary(usize),
}

/// WAM Instructions
#[derive(Debug, Clone)]
pub enum Instruction {
    PutVariable(Register, Register),
    PutValue(Register, Register),
    GetVariable(Register, Register),
    GetValue(Register, Register),
    SetVariable(Register),
    SetValue(Register),
    Call(String),
    Proceed,
    Allocate(usize),
    Deallocate,
    UnifyVariable(Register),
    UnifyValue(Register),
}

/// Environment frame for procedure calls
#[derive(Debug)]
struct Environment {
    permanent_variables: Vec<Term>,
    continuation_pointer: usize,
    previous_environment: Option<Box<Environment>>,
}

/// Choice point frame for backtracking
#[derive(Debug)]
struct ChoicePoint {
    alternative: usize,
    environment: Option<Box<Environment>>,
    trail_pointer: usize,
    arguments: Vec<Term>,
}

/// The Warren Abstract Machine
pub struct Wam {
    heap: Vec<Term>,
    registers: Vec<Term>,
    trail: Vec<usize>,
    environments: Option<Box<Environment>>,
    choice_points: Vec<ChoicePoint>,
    program: Vec<Instruction>,
    program_counter: usize,
    symbol_table: HashMap<String, usize>,
}

impl Wam {
    pub fn new() -> Self {
        Wam {
            heap: Vec::new(),
            registers: vec![Term::Constant("undefined".to_string()); 100],
            trail: Vec::new(),
            environments: None,
            choice_points: Vec::new(),
            program: Vec::new(),
            program_counter: 0,
            symbol_table: HashMap::new(),
        }
    }

    /// Load a program into the WAM
    pub fn load_program(&mut self, instructions: Vec<Instruction>) {
        self.program = instructions;
        self.program_counter = 0;
    }

    /// Execute the next instruction
    pub fn execute_next(&mut self) -> Result<bool, WamError> {
        if self.program_counter >= self.program.len() {
            return Ok(false);
        }

        match &self.program[self.program_counter].clone() {
            Instruction::PutVariable(reg1, reg2) => {
                let var = Term::Variable(format!("V{}", self.heap.len()));
                self.heap.push(var.clone());
                self.set_register(*reg1, var.clone())?;
                self.set_register(*reg2, var)?;
            }
            Instruction::GetVariable(reg1, reg2) => {
                let val = self.get_register(*reg2)?;
                self.set_register(*reg1, val)?;
            }
            Instruction::Call(proc_name) => {
                if let Some(addr) = self.symbol_table.get(proc_name) {
                    self.program_counter = *addr;
                    return Ok(true);
                } else {
                    return Err(WamError::UnknownVariable(proc_name.clone()));
                }
            }
            Instruction::Proceed => {
                if let Some(env) = self.environments.take() {
                    self.program_counter = env.continuation_pointer;
                    self.environments = env.previous_environment;
                    return Ok(true);
                }
            }
            _ => {
                // Implement other instructions
                log::warn!("Unimplemented instruction: {:?}", self.program[self.program_counter]);
            }
        }

        self.program_counter += 1;
        Ok(true)
    }

    /// Get value from a register
    fn get_register(&self, reg: Register) -> Result<Term, WamError> {
        match reg {
            Register::Permanent(n) => {
                if let Some(env) = &self.environments {
                    if n < env.permanent_variables.len() {
                        Ok(env.permanent_variables[n].clone())
                    } else {
                        Err(WamError::StackOverflow)
                    }
                } else {
                    Err(WamError::StackUnderflow)
                }
            }
            Register::Temporary(n) => {
                if n < self.registers.len() {
                    Ok(self.registers[n].clone())
                } else {
                    Err(WamError::StackOverflow)
                }
            }
        }
    }

    /// Set value to a register
    fn set_register(&mut self, reg: Register, value: Term) -> Result<(), WamError> {
        match reg {
            Register::Permanent(n) => {
                if let Some(mut env) = self.environments.take() {
                    if n < env.permanent_variables.len() {
                        env.permanent_variables[n] = value;
                        self.environments = Some(env);
                        Ok(())
                    } else {
                        Err(WamError::StackOverflow)
                    }
                } else {
                    Err(WamError::StackUnderflow)
                }
            }
            Register::Temporary(n) => {
                if n < self.registers.len() {
                    self.registers[n] = value;
                    Ok(())
                } else {
                    Err(WamError::StackOverflow)
                }
            }
        }
    }
}
