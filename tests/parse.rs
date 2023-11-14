#[cfg(test)]
mod parse_tests {
    use props_parser::nodes::{AstNode, Expression, Identifier};
    use props_parser::nodes::MathExpr::{BinaryOp, Literal};
    use props_parser::nodes::MathOp::Mul;
    use props_parser::number::Number::U8;
    use props_parser::PropsParser;

    #[test]
    fn assignment_math() {
        let mut parser = PropsParser::new(r"josh = 2 * 2".to_string());
        let result = parser.parse();
        assert_eq!(result, vec![AstNode::Assignment {
            name: Identifier::Identifier("josh".to_string()),
            expr: Expression::MathExpr(BinaryOp(
                Box::new(Literal(U8(2))),
                Box::new(Literal(U8(2))),
                Mul
            )),
        }]);
    }
    
    #[test]
    fn impure_call() {
        let mut parser = PropsParser::new(r"println hi hi2".to_string());
        let result = parser.parse();
        assert_eq!(result, vec![AstNode::ImpFuncCall {
            name: Identifier::Identifier("println".to_string()),
            arguments: vec![
                Expression::Identifier(Identifier::Identifier("hi".to_string())),
                Expression::Identifier(Identifier::Identifier("hi2".to_string()))
            ]
        }]);
    }

    #[test]
    fn function_call_in_impure_call() {
        let mut parser = PropsParser::new(r"println (add 1 2)".to_string());
        let result = parser.parse();
        assert_eq!(result, vec![AstNode::ImpFuncCall {
            name: Identifier::Identifier("println".to_string()),
            arguments: vec![
                Expression::FuncCall {
                    func_name: Identifier::Identifier("add".to_string()),
                    arguments: vec![
                        Expression::MathExpr(Literal(U8(1))),
                        Expression::MathExpr(Literal(U8(2)))
                    ]
                }
            ]
        }]);
    }
    
    #[test]
    fn str_literal() {
        let mut parser = PropsParser::new("str = \"josh\"".to_string());
        let result = parser.parse();
        assert_eq!(result, vec![AstNode::Assignment {
            name: Identifier::Identifier("str".to_string()),
            expr: Expression::StrLiteral("josh".to_string()),
        }]);
    }

    #[test]
    fn str_literal_argument() {
        let mut parser = PropsParser::new("println \"hello, world!\"".to_string());
        let result = parser.parse();
        assert_eq!(result, vec![AstNode::ImpFuncCall {
            name: Identifier::Identifier("println".to_string()),
            arguments: vec![Expression::StrLiteral("hello, world!".to_string())],
        }]);
    }
}