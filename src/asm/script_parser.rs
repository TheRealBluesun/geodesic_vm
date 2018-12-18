use nom::types::CompleteStr;

use asm::inst_parser::{instruction_one, AsmInstruction};

#[derive(Debug, PartialEq)]
pub struct Script {
    instructions: Vec<AsmInstruction>,
}

named!(pub script<CompleteStr, Script>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Script {
                instructions: instructions
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use asm::Opcode;

    #[test]
    fn test_parse_program() {
        let result = script(CompleteStr("ld r0 i32100\n"));
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
        // TODO: Figure out an ergonomic way to test the AssemblerInstruction returned
    }
}
