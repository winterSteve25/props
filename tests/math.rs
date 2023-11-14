mod ident;

#[cfg(test)]
mod math_tests {
    use props_parser::nodes::MathExpr::{BinaryOp, Literal, Negate};
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
    fn arithmetic_no_parenthesis() {
        let mut parser = PropsParser::new("3 * 2 +4 /2^2".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BinaryOp(
            Box::new(BinaryOp(
                Box::new(Literal(U8(3))),
                Box::new(Literal(U8(2))),
                Mul,
            )),
            Box::new(BinaryOp(
                Box::new(BinaryOp(
                    Box::new(Literal(U8(4))),
                    Box::new(Literal(U8(2))),
                    Div,
                )),
                Box::new(Literal(U8(2))),
                Pow,
            )),
            Add,
        ));
    }

    #[test]
    fn arithmetic() {
        let mut parser = PropsParser::new("3 * (2 + 1)".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BinaryOp(
            Box::new(Literal(U8(3))),
            Box::new(BinaryOp(
                Box::new(Literal(U8(2))),
                Box::new(Literal(U8(1))),
                Add,
            )),
            Mul,
        ));
    }

    #[test]
    fn negation() {
        let mut parser = PropsParser::new("3 * -3".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BinaryOp(
            Box::new(Literal(U8(3))),
            Box::new(Negate(Box::new(Literal(U8(3))))),
            Mul,
        ));
    }

    #[test]
    fn negation_opp_order() {
        let mut parser = PropsParser::new("-3 * 3".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BinaryOp(
            Box::new(Negate(Box::new(Literal(U8(3))))),
            Box::new(Literal(U8(3))),
            Mul,
        ));
    }

    #[test]
    fn negate_expr() {
        let mut parser = PropsParser::new("-(3 * 3)".to_string());
        let result = parser.parse_math_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Negate(Box::new(
            BinaryOp(
                Box::new(Literal(U8(3))),
                Box::new(Literal(U8(3))),
                Mul,
            )
        )));
    }
}