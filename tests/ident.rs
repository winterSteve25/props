#[cfg(test)]
mod ident_tests {
    use props_parser::nodes::Identifier;
    use props_parser::PropsParser;

    #[test]
    fn simple() {
        let mut parser = PropsParser::new("hello".to_string());
        let result = parser.parse_ident();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Identifier::Identifier("hello".to_string()));
    }
    
    #[test]
    fn accessor() {
        let mut parser = PropsParser::new("obj.field".to_string());
        let result = parser.parse_ident();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(), 
            Identifier::Accessor(
                Box::new(Identifier::Identifier("obj".to_string())),
                Box::new(Identifier::Identifier("field".to_string()))
            )
        );
    }
    
    #[test]
    fn compound() {
        let mut parser = PropsParser::new("j1, j2".to_string());
        let result = parser.parse_ident();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Identifier::Compound(vec![
                Identifier::Identifier("j1".to_string()),
                Identifier::Identifier("j2".to_string()),
            ])
        );
    }
    
    #[test]
    fn compounded_accessor() {
        let mut parser = PropsParser::new("obj.field, j2".to_string());
        let result = parser.parse_ident();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Identifier::Compound(vec![
                Identifier::Accessor(
                    Box::new(Identifier::Identifier("obj".to_string())),
                    Box::new(Identifier::Identifier("field".to_string()))
                ),
                Identifier::Identifier("j2".to_string()),
            ])
        );
    }
}