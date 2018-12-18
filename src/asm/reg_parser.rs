use nom::types::CompleteStr;
use nom::digit;
use asm::Token;

named!(pub register<CompleteStr, Token>,
    ws!( 
        do_parse!(
            tag!("r") >>
            reg_num: digit >>
            ( 
                Token::Reg{ 
                  reg_num: reg_num.parse::<u8>().unwrap() 
                } 
            ) 
        )
    )
);

#[cfg(test)]
mod tests {
use super::*;


#[test]
  fn test_parse_register() {
      let result = register(CompleteStr("r0"));
      assert_eq!(result.is_ok(), true);
      let result = register(CompleteStr("0"));
      assert_eq!(result.is_ok(), false);
      let result = register(CompleteStr("ra"));
      assert_eq!(result.is_ok(), false);
  }
}