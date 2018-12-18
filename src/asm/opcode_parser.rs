use nom::types::CompleteStr;
use asm::Token;
use instruction::Opcode;

named!(pub opcode_load<CompleteStr, Token>,
  do_parse!(
      tag!("ld") >> (Token::Op{code: Opcode::LOD})
  )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode_load(CompleteStr("ld"));
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOD});
        assert_eq!(rest, CompleteStr(""));

        // Tests that an invalid opcode isn't recognized
        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}