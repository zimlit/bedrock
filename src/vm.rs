use crate::instruction::Opcode;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum CmpRes {
    Eq,
    Gt,
    Lt,
    Neq,
    No,
}

#[derive(Debug)]
pub struct MemBlock {
    pub length: u64,
    pub data: Vec<Val>,
}

impl MemBlock {
    pub fn new() -> MemBlock {
	MemBlock{
	    length: 0,
	    data: Vec::new()
	}
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Val {
    Int(i64),
    Ptr(u64),
}

impl Val {
    pub fn as_int(&self) -> i64 {
	match self {
	    Val::Int(v) => return *v,
	    Val::Ptr(v) => return *v as i64,
	}
    }
    pub fn as_uint(&self) -> u64 {
	match self {
	    Val::Int(v) => return *v as u64,
	    Val::Ptr(v) => return *v,
	}
    }
}

pub struct VM {
    pub registers: [Val; 256],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u64,
    pub heap: HashMap<u64, MemBlock>,
    equal_flag: CmpRes,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [Val::Int(0); 256],
            pc: 0,
            program: vec![],
	    remainder: 0,
	    equal_flag: CmpRes::No,
	    heap: HashMap::new(),
        }
    }

    fn next_8_bits(&mut self) -> u8 {
	let result = self.program[self.pc];
	self.pc += 1;
	result
    }

    fn get_int(&mut self) -> i64 {
	let result = ((self.program[self.pc] as u64) << 56) | ((self.program[self.pc + 1] as u64) << 48) | ((self.program[self.pc + 2] as u64) << 40) | ((self.program[self.pc + 3] as u64) << 32) | ((self.program[self.pc + 4] as u64) << 24) | ((self.program[self.pc + 5] as u64) << 16) | ((self.program[self.pc + 6] as u64) << 8) | self.program[self.pc + 7] as u64;
	self.pc += 8;
	let sign = (result & (0b1 << 63)) >> 63;
	let res = (result & (0b0111111111111111111111111111111111111111111111111111111111111111)) as i64;
	if sign > 0 {
	    return -res;
	} else {
	    res
	}
    }
    
    fn get_uint(&mut self) -> u64 {
	let result = ((self.program[self.pc] as u64) << 56) | ((self.program[self.pc + 1] as u64) << 48) | ((self.program[self.pc + 2] as u64) << 40) | ((self.program[self.pc + 3] as u64) << 32) | ((self.program[self.pc + 4] as u64) << 24) | ((self.program[self.pc + 5] as u64) << 16) | ((self.program[self.pc + 6] as u64) << 8) | self.program[self.pc + 7] as u64;
	self.pc += 8;
	result
    }

    pub fn run(&mut self) {
	let mut is_done = false;
	while !is_done {
            is_done = self.execute_instruction();
	}
    }

    pub fn execute_instruction(&mut self) -> bool {
	
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
		let number = self.get_int();
		self.registers[register] = Val::Int(number); // Our registers are i32s, so we need to cast it. We'll cover that later.
	    },
	    Opcode::Add => {
		let r1 = self.registers[self.next_8_bits() as usize];
		let r2 = self.registers[self.next_8_bits() as usize];
		self.registers[self.next_8_bits() as usize] = Val::Int(r1.as_int() + r2.as_int());
	    },
	    Opcode::Sub => {
		let r1 = self.registers[self.next_8_bits() as usize];
		let r2 = self.registers[self.next_8_bits() as usize];
		self.registers[self.next_8_bits() as usize] = Val::Int(r1.as_int() - r2.as_int());
	    },
	    Opcode::Mul => {
		let r1 = self.registers[self.next_8_bits() as usize];
		let r2 = self.registers[self.next_8_bits() as usize];
		self.registers[self.next_8_bits() as usize] = Val::Int(r1.as_int() * r2.as_int());
	    },
	    Opcode::Div => {
		let r1 = self.registers[self.next_8_bits() as usize];
		let r2 = self.registers[self.next_8_bits() as usize];
		self.registers[self.next_8_bits() as usize] = Val::Int(r1.as_int() / r2.as_int());
		self.remainder = (r1.as_int() % r1.as_int()) as u64;
	    },
	    Opcode::Jmp => {
		let t = self.registers[self.next_8_bits() as usize];
		self.pc = t.as_uint() as usize;
	    },
	    Opcode::Jmpf => {
		let v = self.registers[self.next_8_bits() as usize];
		self.pc += v.as_uint() as usize;
	    },
	    Opcode::Jmpb => {
		let v = self.registers[self.next_8_bits() as usize];
		self.pc -= v.as_uint() as usize;
	    },
	    Opcode::Cmp => {
		let r1 = self.registers[self.next_8_bits() as usize];
		let r2 = self.registers[self.next_8_bits() as usize];
		if r1.as_int() == r2.as_int() {
		    self.equal_flag = CmpRes::Eq;
		} else if r1.as_int() > r2.as_int() {
		    self.equal_flag = CmpRes::Gt;
		} else if r1.as_int() < r2.as_int() {
		    self.equal_flag = CmpRes::Lt;
		} else {
		    self.equal_flag = CmpRes::Neq;
		}
	    },
	    Opcode::Jeq => {
		let t = self.registers[self.next_8_bits() as usize];
		if self.equal_flag == CmpRes::Eq {
		    self.pc = t.as_uint() as usize;
		}
	    },
	    Opcode::Jne => {
		let t = self.registers[self.next_8_bits() as usize];
		if self.equal_flag == CmpRes::Neq {
		    self.pc = t.as_uint() as usize;
		}
	    },
	    Opcode::Jgt => {
		let t = self.registers[self.next_8_bits() as usize];
		if self.equal_flag == CmpRes::Gt {
		    self.pc = t.as_uint() as usize;
		}
	    },
	    Opcode::Jlt => {
		let t = self.registers[self.next_8_bits() as usize];
		if self.equal_flag == CmpRes::Lt {
		    self.pc = t.as_uint() as usize;
		}
	    },
	    Opcode::Jgq => {
		let t = self.registers[self.next_8_bits() as usize];
		if self.equal_flag == CmpRes::Gt || self.equal_flag == CmpRes::Eq {
		    self.pc = t.as_uint() as usize;
		}
	    },
	    Opcode::Jlq => {
		let t = self.registers[self.next_8_bits() as usize];
		if self.equal_flag == CmpRes::Lt || self.equal_flag == CmpRes::Eq {
		    self.pc = t.as_uint() as usize;
		}
	    },
	    Opcode::Write => {
		let b_addr = self.next_8_bits();
		let block = self.registers[b_addr as usize];
		let offset = self.registers[self.next_8_bits() as usize];
		let v = self.get_int();
		self.registers[b_addr as usize] = Val::Ptr(block.as_uint() as u64);
		if self.heap.contains_key(&(block.as_uint() as u64)) {
		    if let Some(k) = self.heap.get_mut(&(block.as_uint() as u64)) {
			while k.data.len() <= offset.as_uint() as usize {
			    k.data.push(Val::Int(0));
			}
			k.data[offset.as_uint() as usize] = Val::Int(v);
			k.length = k.data.len() as u64;
		    }
		} else {
		    self.heap.insert(block.as_uint() as u64, MemBlock::new());
		    
		    if let Some(k) = self.heap.get_mut(&(block.as_uint() as u64)) {
			while k.data.len() <= offset.as_uint() as usize {
			    k.data.push(Val::Int(0));
			}
			k.data[offset.as_uint() as usize] = Val::Int(v);
			k.length = k.data.len() as u64;
		    }
		}
	    },
	    Opcode::WritePtr => {
		let b_addr = self.next_8_bits();
		let block = self.registers[b_addr as usize];
		let offset = self.registers[self.next_8_bits() as usize];
		let v = self.get_uint();
		self.registers[b_addr as usize] = Val::Ptr(block.as_uint() as u64);
		if self.heap.contains_key(&(block.as_uint() as u64)) {
		    if let Some(k) = self.heap.get_mut(&(block.as_uint() as u64)) {
			while k.data.len() <= offset.as_uint() as usize {
			    k.data.push(Val::Int(0));
			}
			k.data[offset.as_uint() as usize] = Val::Ptr(v);
			k.length = k.data.len() as u64;
		    }
		} else {
		    self.heap.insert(block.as_uint() as u64, MemBlock::new());
		    
		    if let Some(k) = self.heap.get_mut(&(block.as_uint() as u64)) {
			while k.data.len() <= offset.as_uint() as usize {
			    k.data.push(Val::Int(0));
			}
			k.data[offset.as_uint() as usize] = Val::Ptr(v);
			k.length = k.data.len() as u64;
		    }
		}
	    },
	    Opcode::Loadptr => {
		let register = self.next_8_bits() as usize; // We cast to usize so we can use it as an index into the array
		let number = self.get_uint();
		self.registers[register] = Val::Ptr(number as u64); // Our registers are i32s, so we need to cast it. We'll cover that later.
	    },
	    Opcode::Deref => {
		let b_addr = self.next_8_bits();
		let block = self.registers[b_addr as usize];
		let offset = self.registers[self.next_8_bits() as usize];
		let target = self.next_8_bits() as usize;
		self.registers[b_addr as usize] = Val::Ptr(block.as_uint() as u64);
		if let Some(v) = self.heap.get(&(block.as_uint())) {
		    self.registers[target] = v.data[offset.as_uint() as usize];
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
        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], Val::Int(0))
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
	test_vm.program = vec![1, 0, 0b10000000, 0, 0, 0, 0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[0], Val::Int(-500));
    }

    #[test]
    fn test_add_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], Val::Int(502));
    }
    #[test]
    fn test_sub_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], Val::Int(498));
    }
    #[test]
    fn test_mul_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 4, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], Val::Int(1000));
    }
    #[test]
    fn test_div_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 244, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 5, 0, 1, 2]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[2], Val::Int(250));
    }
    #[test]
    fn test_jmp_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = Val::Int(1);
	test_vm.program = vec![6, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 1);
    }
    
    #[test]
    fn test_jmpf_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = Val::Int(1);
	test_vm.program = vec![7, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 3);
    }
    #[test]
    fn test_jmpb_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = Val::Int(1);
	test_vm.program = vec![8, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_cmp_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = Val::Int(1);
	test_vm.registers[1] = Val::Int(3);
	test_vm.program = vec![9, 0, 1];
	test_vm.execute_instruction();
	assert_eq!(test_vm.equal_flag, CmpRes::Lt);
    }
    #[test]
    fn test_jeq_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[0] = Val::Int(7);
	test_vm.equal_flag = CmpRes::Eq;
	test_vm.program = vec![10, 0];
	test_vm.execute_instruction();
	assert_eq!(test_vm.pc, 7);
    }
    #[test]
    fn test_write_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[1] = Val::Int(1);
	test_vm.program = vec![16, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2];
	test_vm.execute_instruction();
	if let Some(x) = test_vm.heap.get(&1) {
	    assert_eq!(x.data[1], Val::Int(2));
	} else {
	    panic!("missing block");
	}
    }
    #[test]
    fn test_writeptr_opcode() {
	let mut test_vm = VM::new();
	test_vm.registers[1] = Val::Int(1);
	test_vm.program = vec![17, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2];
	test_vm.execute_instruction();
	if let Some(x) = test_vm.heap.get(&1) {
	    assert_eq!(x.data[1], Val::Ptr(2));
	} else {
	    panic!("missing block");
	}
    }
    #[test]
    fn test_loadptr_opcode() {
	let mut test_vm = VM::new();
	test_vm.program = vec![18, 0, 0, 0, 0, 0, 0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
	test_vm.run();
	assert_eq!(test_vm.registers[0], Val::Ptr(500));
    }
}
