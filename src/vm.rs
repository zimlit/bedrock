use crate::instruction::Opcode;

#[derive(Debug, Eq, PartialEq)]
pub enum CmpRes {
    Eq,
    Gt,
    Lt,
    Neq,
    No,
}

pub struct VM {
    registers: [i64; 256],
    pc: usize,
    program: Vec<u8>,
    remainder: u64,
    equal_flag: CmpRes,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 256],
            pc: 0,
            program: vec![],
	    remainder: 0,
	    equal_flag: CmpRes::No,
        }
    }

    fn next_8_bits(&mut self) -> u8 {
	let result = self.program[self.pc];
	self.pc += 1;
	return result;
    }

    fn next_32_bits(&mut self) -> u32 {
	let result = ((self.program[self.pc] as u32) << 24) | ((self.program[self.pc + 1] as u32) << 16) | ((self.program[self.pc + 2] as u32) << 8) | self.program[self.pc + 3] as u32;
	self.pc += 4;
	return result;
    }
   

    pub fn run(&mut self) {
	let mut is_done = false;
	while !is_done {
            is_done = self.execute_instruction();
	}
    }

    fn execute_instruction(&mut self) -> bool {
	
            if self.pc >= self.program.len() {
                return true;
            }
            match self.decode_opcode() {
                Opcode::Hlt => {
                    println!("hlt encountered");
		    return true;
                },
		Opcode::Load => {
		    let register = self.next_8_bits() as usize; // We cast to usize so we can use it as an index into the array
		    let number = self.next_32_bits() as u32;
		    self.registers[register] = number as i64; // Our registers are i32s, so we need to cast it. We'll cover that later.
		},
		Opcode::Add => {
		    let r1 = self.registers[self.next_8_bits() as usize];
		    let r2 = self.registers[self.next_8_bits() as usize];
		    self.registers[self.next_8_bits() as usize] = r1 + r2;
		},
		Opcode::Sub => {
		    let r1 = self.registers[self.next_8_bits() as usize];
		    let r2 = self.registers[self.next_8_bits() as usize];
		    self.registers[self.next_8_bits() as usize] = r1 - r2;
		},
		Opcode::Mul => {
		    let r1 = self.registers[self.next_8_bits() as usize];
		    let r2 = self.registers[self.next_8_bits() as usize];
		    self.registers[self.next_8_bits() as usize] = r1 * r2;
		},
		Opcode::Div => {
		    let r1 = self.registers[self.next_8_bits() as usize];
		    let r2 = self.registers[self.next_8_bits() as usize];
		    self.registers[self.next_8_bits() as usize] = r1 / r2;
		    self.remainder = (r1 % r2) as u64;
		},
		Opcode::Jmp => {
		    let t = self.registers[self.next_8_bits() as usize];
		    self.pc = t as usize;
		},
		Opcode::Jmpf => {
		    let v = self.registers[self.next_8_bits() as usize];
		    self.pc += v as usize;
		},
		Opcode::Jmpb => {
		    let v = self.registers[self.next_8_bits() as usize];
		    self.pc -= v as usize;
		},
		Opcode::Cmp => {
		    let r1 = self.registers[self.next_8_bits() as usize];
		    let r2 = self.registers[self.next_8_bits() as usize];
		    if r1 == r2 {
			self.equal_flag = CmpRes::Eq;
		    } else if r1 > r2 {
			self.equal_flag = CmpRes::Gt;
		    } else if r1 < r2 {
			self.equal_flag = CmpRes::Lt;
		    } else {
			self.equal_flag = CmpRes::Neq;
		    }
		},
		Opcode::Jeq => {
		    let t = self.registers[self.next_8_bits() as usize];
		    if self.equal_flag == CmpRes::Eq {
			self.pc = t as usize;
		    }
		},
		Opcode::Jne => {
		    let t = self.registers[self.next_8_bits() as usize];
		    if self.equal_flag == CmpRes::Neq {
			self.pc = t as usize;
		    }
		},
		Opcode::Jgt => {
		    let t = self.registers[self.next_8_bits() as usize];
		    if self.equal_flag == CmpRes::Gt {
			self.pc = t as usize;
		    }
		},
		Opcode::Jlt => {
		    let t = self.registers[self.next_8_bits() as usize];
		    if self.equal_flag == CmpRes::Lt {
			self.pc = t as usize;
		    }
		},
		Opcode::Jgq => {
		    let t = self.registers[self.next_8_bits() as usize];
		    if self.equal_flag == CmpRes::Gt || self.equal_flag == CmpRes::Eq {
			self.pc = t as usize;
		    }
		},
		Opcode::Jlq => {
		    let t = self.registers[self.next_8_bits() as usize];
		    if self.equal_flag == CmpRes::Lt || self.equal_flag == CmpRes::Eq {
			self.pc = t as usize;
		    }
		},
		_ => {
		    return true;
		}
	    }
	false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
      let mut test_vm = VM::new();
      let test_bytes = vec![0];
      test_vm.program = test_bytes;
      test_vm.run();
      assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
      let mut test_vm = VM::new();
      let test_bytes = vec![200,0,0,0];
      test_vm.program = test_bytes;
      test_vm.run();
      assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_load_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 2, 2, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], 502);
    }
    #[test]
    fn test_sub_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 2, 3, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], 498);
    }
    #[test]
    fn test_mul_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 2, 4, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], 1000);
    }
    #[test]
    fn test_div_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 2, 5, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], 250);
    }
    #[test]
    fn test_jmp_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = 1;
	test_vm.program = vec![6, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 1);
    }
   
    #[test]
    fn test_jmpf_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = 1;
	test_vm.program = vec![7, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 3);
    }
    #[test]
    fn test_jmpb_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = 1;
	test_vm.program = vec![8, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_cmp_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = 1;
	test_vm.registers[1] = 3;
	test_vm.program = vec![9, 0, 1];
	test_vm.execute_instruction();
	assert_eq!(test_vm.equal_flag, CmpRes::Lt);
    }
    #[test]
    fn test_jeq_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = 7;
	test_vm.equal_flag = CmpRes::Eq;
	test_vm.program = vec![10, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 7);
    }
}
