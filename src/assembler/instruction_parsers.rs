use crate::assembler::Token;
use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::operand;
use nom::types::CompleteStr;
use nom::multispace;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    results.push(code as u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

	for operand in &[&self.operand1, &self.operand2, &self.operand3] {
	    if let Some(token) = operand {
		AssemblerInstruction::extract_operand(token, &mut results)
	    }
	}
        results
    }
    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
	match t {
            Token::Register { reg_num } => {
		results.push(*reg_num);
            }
            Token::Pos { value } => {
		let converted = *value;
		let byte1 = converted;
		let byte2 = converted >> 8;
		let byte3 = converted >> 16;
		let byte4 = converted >> 24;
		let byte5 = converted >> 32;
		let byte6 = converted >> 40;
		let byte7 = converted >> 48;
		let byte8 = converted >> 56;
		results.push(byte8 as u8);
		results.push(byte7 as u8);
		results.push(byte6 as u8);
		results.push(byte5 as u8);
		results.push(byte4 as u8);
		results.push(byte3 as u8);
		results.push(byte2 as u8);
		results.push(byte1 as u8);
            },
            Token::Neg { value } => {
		let converted = (*value).abs() as u64;
		let byte1 = converted;
		let byte2 = converted >> 8;
		let byte3 = converted >> 16;
		let byte4 = converted >> 24;
		let byte5 = converted >> 32;
		let byte6 = converted >> 40;
		let byte7 = converted >> 48;
		let byte8 = ((converted >> 56) | (1 << 63)) >> 56;
		results.push(byte8 as u8);
		results.push(byte7 as u8);
		results.push(byte6 as u8);
		results.push(byte5 as u8);
		results.push(byte4 as u8);
		results.push(byte3 as u8);
		results.push(byte2 as u8);
		results.push(byte1 as u8);
            },
            _ => {
		println!("Opcode found in operand field");
		std::process::exit(1);
            }
	};
    }
}

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
	opt!(multispace) >>    
        (
            AssemblerInstruction{
                opcode: o,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);


named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
	   instruction_combined 
        ) >>
        (
            ins
        )
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction(CompleteStr("load r0 100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::Load },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Pos { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
    #[test]
    fn test_parse_instruction_form_two() {
	let result = instruction(CompleteStr("hlt\n"));
	assert_eq!(
            result,
            Ok((
		CompleteStr(""),
		AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::Hlt },
                    operand1: None,
                    operand2: None,
                    operand3: None
		}
            ))
	);
    }

    #[test]
    fn test_parse_instruction_form_three() {
	let result = instruction(CompleteStr("add r0 r1 r2\n"));
	assert_eq!(
            result,
            Ok((
		CompleteStr(""),
		AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::Add },
                    operand1: Some(Token::Register {reg_num: 0}),
                    operand2: Some(Token::Register {reg_num: 1}),
                    operand3: Some(Token::Register {reg_num: 2}),
		}
            ))
	);
    }
}
