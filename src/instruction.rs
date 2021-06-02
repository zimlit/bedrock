use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    Write,
    WritePtr,
    Loadptr,
    Deref,
    Igl,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
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
	    16 => return Opcode::Write,
	    17 => return Opcode::WritePtr,
	    18 => return Opcode::Loadptr,
	    19 => return Opcode::Deref,
	    _ => return Opcode::Igl,
        }
    }
}
impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr("load") => Opcode::Load,
            CompleteStr("add") => Opcode::Add,
            CompleteStr("sub") => Opcode::Sub,
            CompleteStr("mul") => Opcode::Mul,
            CompleteStr("div") => Opcode::Div,
            CompleteStr("hlt") => Opcode::Hlt,
            CompleteStr("jmp") => Opcode::Jmp,
            CompleteStr("jmpf") => Opcode::Jmpf,
            CompleteStr("jmpb") => Opcode::Jmpb,
	    CompleteStr("cmp") => Opcode::Cmp,
	    CompleteStr("jeq") => Opcode::Jeq,
	    CompleteStr("jne") => Opcode::Jne,
	    CompleteStr("jgt") => Opcode::Jgt,
	    CompleteStr("jlt") => Opcode::Jlt,
	    CompleteStr("jgq") => Opcode::Jgq,
	    CompleteStr("jlq") => Opcode::Jlq,
	    CompleteStr("write") => Opcode::Write,
	    CompleteStr("writeptr") => Opcode::WritePtr,
	    CompleteStr("loadptr") => Opcode::Loadptr,
	    CompleteStr("deref") => Opcode::Deref,
            _ => Opcode::Igl,
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

