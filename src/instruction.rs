#[derive(Debug, PartialEq)]
pub enum Opcode {
    Hlt,
    Load,
    Add,
    Sub,
    Mul,
    Div,
    Jmp,
    Jmpf,
    Jmpb,
    Cmp,
    Jeq,
    Jne,
    Jgt,
    Jlt,
    Jgq,
    Jlq,
    Igl,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::Hlt,
	    1 => return Opcode::Load,
	    2 => return Opcode::Add,
	    3 => return Opcode::Sub,
	    4 => return Opcode::Mul,
	    5 => return Opcode::Div,
	    6 => return Opcode::Jmp,
	    7 => return Opcode::Jmpf,
	    8 => return Opcode::Jmpb,
	    9 => return Opcode::Cmp,
	    10 => return Opcode::Jeq,
	    11 => return Opcode::Jne,
	    12 => return Opcode::Jgt,
	    13 => return Opcode::Jlt,
	    14 => return Opcode::Jgq,
	    15 => return Opcode::Jlq,
	    _ => return Opcode::Igl,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::Hlt;
        assert_eq!(opcode, Opcode::Hlt);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::Hlt);
        assert_eq!(instruction.opcode, Opcode::Hlt);
    }
}

