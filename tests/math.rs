#[cfg(test)]
mod test {
    use props_parser::nodes::MathExpr::{BinaryOp, Literal};
    use props_parser::nodes::MathOp::{Add, Div, Mul, Pow, Sub};
    use props_parser::PropsParser;
    use props_parser::tokens::Number::U8;

    #[test]
    fn add_with_empty() {
        let mut parser = PropsParser::new("         10 +  5".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BinaryOp(
            Box::new(Literal(U8(10))),
            Box::new(Literal(U8(5))),
            Add));
    }

    #[test]
    fn add_fail() {
        let mut parser = PropsParser::new("10 ++ 5".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_err());
    }

    #[test]
    fn mul_with_empty() {
        let mut parser = PropsParser::new("   2 - 10 * 5 + 2".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),
                   BinaryOp(
                       Box::new(
                           BinaryOp(
                               Box::new(Literal(U8(2))),
                               Box::new(BinaryOp(
                                   Box::new(Literal(U8(10))),
                                   Box::new(Literal(U8(5))),
                                   Mul,
                               )),
                               Sub,
                           )
                       ),
                       Box::new(Literal(U8(2))),
                       Add,
                   )
        );
    }

    #[test]
    fn arithmetic() {
        let mut parser = PropsParser::new("3 * 2 +4/2^2".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BinaryOp(
            Box::new(BinaryOp(
                Box::new(Literal(U8(3))),
                Box::new(Literal(U8(2))),
                Mul
            )),
            Box::new(BinaryOp(
                Box::new(BinaryOp(
                   Box::new(Literal(U8(4))),
                   Box::new(Literal(U8(2))),
                    Div
                )),
                Box::new(Literal(U8(2))),
                Pow
            )),
            Add
        ));
    }
}