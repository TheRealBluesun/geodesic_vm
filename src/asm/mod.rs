use instruction::Opcode;
pub mod arg_parser;
pub mod opcode_parser;
pub mod reg_parser;
pub mod inst_parser;
pub mod script_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Reg { reg_num: u8 },
    Number { value: i32 },
}
