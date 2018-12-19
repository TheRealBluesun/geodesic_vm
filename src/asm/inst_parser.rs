use nom::types::CompleteStr;
use asm::Token;
use asm::opcode_parser::opcode_load;
use asm::arg_parser::i32_arg;
use asm::reg_parser::register;

#[derive(Debug, PartialEq)]
pub struct AsmInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AsmInstruction {
    pub fn to_bytes(self) -> Vec<u8> {
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
            match operand {
                Some(t) => AsmInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
    match t {
        Token::Reg { reg_num } => {
            results.push(*reg_num);
        }
        Token::Number { value } => {
            let converted = *value as u16;
            let byte1 = converted;
            let byte2 = converted >> 8;
            results.push(byte2 as u8);
            results.push(byte1 as u8);
        }
        _ => {
            println!("Opcode found in operand field");
            std::process::exit(1);
        }
    };
}

}

/// Handles instructions of the following form:
/// LOAD $0 #100
named!(pub instruction_one<CompleteStr, AsmInstruction>,
    do_parse!(
        o: opcode_load >>
        r: register >>
        i: i32_arg >>
        (
            AsmInstruction{
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
            }
        )
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    use asm::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("ld r0 i32100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AsmInstruction {
                    //label: None,
                    opcode: Token::Op { code: Opcode::LOD },
                    operand1: Some(Token::Reg { reg_num: 0 }),
                    operand2: Some(Token::Number { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}