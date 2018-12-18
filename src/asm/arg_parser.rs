use nom::digit;
use nom::types::CompleteStr;
use asm::Token;

/// Parser for integer numbers, which we preface with `#` in our assembly language:
/// #100
named!(pub i32_arg<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("i32") >>
            val: digit >>
            (
                Token::Number{value: val.parse::<i32>().unwrap()}
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i32_arg() {
        // Test a valid integer operand
        let result = i32_arg(CompleteStr("i3210"));
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Number { value: 10 });

        // Test an invalid one (missing the #)
        let result = i32_arg(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }
} /*  */
