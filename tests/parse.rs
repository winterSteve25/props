#[cfg(test)]
mod parse_tests {
    use props_parser::nodes::{AstNode, Expression, Identifier};
    use props_parser::nodes::MathExpr::{BinaryOp, Literal};
    use props_parser::nodes::MathOp::Mul;
    use props_parser::types::Number::U8;
    use props_parser::PropsParser;
    use props_parser::types::Type;

    #[test]
    fn assignment_math() {
        let mut parser = PropsParser::new(r"josh = 2 * 2".to_string());
        let result = parser.parse();
        assert_eq!(result.0, vec![AstNode::Assignment(
            Identifier::Identifier("josh".to_string(), Type::None),
            Expression::MathExpr(BinaryOp(
                Box::new(Literal(U8(2))),
                Box::new(Literal(U8(2))),
                Mul
            )),
        )]);
    }

    #[test]
    fn impure_call() {
        let mut parser = PropsParser::new(r"println hi hi2".to_string());
        let result = parser.parse();
        assert_eq!(result.0, vec![AstNode::ImpFuncCall(
            Identifier::Identifier("println".to_string(), Type::None),
            vec![
                Expression::Identifier(Identifier::Identifier("hi".to_string(), Type::None)),
                Expression::Identifier(Identifier::Identifier("hi2".to_string(), Type::None))
            ],
        )]);
    }

    #[test]
    fn function_call_in_impure_call() {
        let mut parser = PropsParser::new(r"println (add 1 2)".to_string());
        let result = parser.parse();
        assert_eq!(result.0, vec![AstNode::ImpFuncCall(
            Identifier::Identifier("println".to_string(), Type::None),
            vec![
                Expression::FuncCall(
                    Identifier::Identifier("add".to_string(), Type::None),
                    vec![
                        Expression::MathExpr(Literal(U8(1))),
                        Expression::MathExpr(Literal(U8(2)))
                    ],
                ),
            ],
        )]);
    }

    #[test]
    fn str_literal() {
        let mut parser = PropsParser::new("str = \"josh\"".to_string());
        let result = parser.parse();
        assert_eq!(result.0, vec![AstNode::Assignment(
            Identifier::Identifier("str".to_string(), Type::None),
            Expression::StrLiteral("josh".to_string()),
        )]);
    }

    #[test]
    fn str_literal_argument() {
        let mut parser = PropsParser::new("println \"hello, world!\"".to_string());
        let result = parser.parse();
        assert_eq!(result.0, vec![AstNode::ImpFuncCall(
            Identifier::Identifier("println".to_string(), Type::None),
            vec![Expression::StrLiteral("hello, world!".to_string())],
        )]);
    }

    #[test]
    fn typed_assignment() {
        let mut parser = PropsParser::new("number: I32 = 32".to_string());
        let result = parser.parse();
        assert_eq!(result.0, vec![AstNode::Assignment(
            Identifier::Identifier("number".to_string(), Type::Defined("I32".to_string())),
            Expression::MathExpr(Literal(U8(32))),
        )]);
    }

    #[test]
    fn invalid_typed_accessor() {
        let mut parser = PropsParser::new("obj.field: I32 = 32".to_string());
        let result = parser.parse();
        assert_eq!(result.1.len(), 1);
    }
}