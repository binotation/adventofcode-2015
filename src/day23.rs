use std::str::FromStr;

/// u8 = reg, i32 = offset
#[allow(dead_code)]
enum Instruction {
    Hlf(u8),
    Tpl(u8),
    Inc(u8),
    Jmp(i32),
    Jie(u8, i32),
    Jio(u8, i32),
}

#[allow(dead_code)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(instructions_str: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for instruction in instructions_str.lines() {
            if instruction.starts_with("hlf") {
                let mut instruction_split = instruction.split(" ");
                let reg = instruction_split.nth(1).unwrap().as_bytes()[0];
                instructions.push(Instruction::Hlf(reg));
            } else if instruction.starts_with("tpl") {
                let mut instruction_split = instruction.split(" ");
                let reg = instruction_split.nth(1).unwrap().as_bytes()[0];
                instructions.push(Instruction::Tpl(reg));
            } else if instruction.starts_with("inc") {
                let mut instruction_split = instruction.split(" ");
                let reg = instruction_split.nth(1).unwrap().as_bytes()[0];
                instructions.push(Instruction::Inc(reg));
            } else if instruction.starts_with("jmp") {
                let mut instruction_split = instruction.split(" ");
                let offset = instruction_split.nth(1).unwrap().parse::<i32>().unwrap();
                instructions.push(Instruction::Jmp(offset));
            } else if instruction.starts_with("jie") {
                let mut instruction_split = instruction.split(" ");
                let reg = instruction_split.nth(1).unwrap().as_bytes()[0];
                let offset = instruction_split.next().unwrap().parse::<i32>().unwrap();
                instructions.push(Instruction::Jie(reg, offset));
            } else if instruction.starts_with("jio") {
                let mut instruction_split = instruction.split(" ");
                let reg = instruction_split.nth(1).unwrap().as_bytes()[0];
                let offset = instruction_split.next().unwrap().parse::<i32>().unwrap();
                instructions.push(Instruction::Jio(reg, offset));
            } else {
                unreachable!();
            }
        }
        Ok(Program { instructions })
    }
}

#[allow(dead_code)]
impl Program {
    fn offset_pc(pc: usize, offset: i32) -> usize {
        if offset < 0 {
            pc.wrapping_sub(offset.unsigned_abs().try_into().unwrap())
        } else {
            pc + (offset as usize)
        }
    }

    fn execute(&self, a: u32) -> u32 {
        let mut a: u32 = a;
        let mut b: u32 = 0;
        let mut pc: usize = 0;
        while pc < self.instructions.len() {
            match self.instructions[pc] {
                Instruction::Hlf(reg) => match reg {
                    b'a' => a /= 2,
                    b'b' => b /= 2,
                    _ => unreachable!(),
                },
                Instruction::Tpl(reg) => match reg {
                    b'a' => a *= 3,
                    b'b' => b *= 3,
                    _ => unreachable!(),
                },
                Instruction::Inc(reg) => match reg {
                    b'a' => a += 1,
                    b'b' => b += 1,
                    _ => unreachable!(),
                },
                Instruction::Jmp(offset) => {
                    pc = Self::offset_pc(pc, offset);
                    continue;
                }
                Instruction::Jie(reg, offset) => {
                    let is_even = match reg {
                        b'a' => a % 2 == 0,
                        b'b' => b % 2 == 0,
                        _ => unreachable!(),
                    };
                    if is_even {
                        pc = Self::offset_pc(pc, offset);
                        continue;
                    }
                }
                Instruction::Jio(reg, offset) => {
                    let is_one = match reg {
                        b'a' => a == 1,
                        b'b' => b == 1,
                        _ => unreachable!(),
                    };
                    if is_one {
                        pc = Self::offset_pc(pc, offset);
                        continue;
                    }
                }
            }
            pc += 1;
        }
        b
    }
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn get_reg_b_after_execute() {
        let program = get_input("program").unwrap();
        let program: Program = program.parse().unwrap();
        let b = program.execute(0);
        assert_eq!(b, 184);
    }

    #[test]
    fn get_reg_b_after_execute_reg_a_initialize_1() {
        let program = get_input("program").unwrap();
        let program: Program = program.parse().unwrap();
        let b = program.execute(1);
        assert_eq!(b, 231);
    }
}
