#[cfg(test)]
mod test {
    use props_parser::nodes::MathExpr::{BinaryOp, Literal};
    use props_parser::nodes::MathOp::Add;
    use props_parser::PropsParser;
    use props_parser::tokens::Number::U8;

    #[test]
    fn add() {
        let mut parser = PropsParser::new("10 +5".to_string());
        let result = parser.parse_additive_expr();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BinaryOp(
            Box::new(Literal(U8(10))),
            Box::new(Literal(U8(5))),
            Add));
    }
}