use nom::types::CompleteStr;
use nom::digit;

use crate::assembler::Token;
use crate::assembler::register_parsers::register;

named!(pub pos<CompleteStr, Token>,
       ws!(
           do_parse!(
	       num: digit >>
		   (
		       Token::Pos{value: num.parse::<u64>().unwrap()}
			   )
           )
       )
);
named!(pub neg<CompleteStr, Token>,
       ws!(
           do_parse!(
	       _s: tag!("-") >>
	       num: digit >>
		   (
		       Token::Neg{value: -(num.parse::<i64>().unwrap())}
			   )
           )
       )
);
named!(pub integer_operand<CompleteStr, Token>,
       ws!(
           do_parse!(
	       n: alt!(
		   neg |
		   pos
	       ) >>
		   (
		       n
		   )
           )
       )
);

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        register
    )
);


mod test {
    use super::*;
    #[test]
    fn test_parse_integer_operand() {
	// Test a valid integer operand
	let result = integer_operand(CompleteStr("10"));
	assert_eq!(result.is_ok(), true);
	let (rest, value) = result.unwrap();
	assert_eq!(rest, CompleteStr(""));
	assert_eq!(value, Token::Pos{value: 10});

	let result = integer_operand(CompleteStr("-10"));
	assert_eq!(result.is_ok(), true);
	let (rest, value) = result.unwrap();
	assert_eq!(rest, CompleteStr(""));
	assert_eq!(value, Token::Neg{value: -10});
	// Test an invalid one (missing the #)
	let result = integer_operand(CompleteStr("#10"));
	assert_eq!(result.is_ok(), false);
    }
}
