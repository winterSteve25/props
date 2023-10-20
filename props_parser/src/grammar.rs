#[rust_sitter::grammar("Props")]
pub mod grammar {
    
    #[rust_sitter::language]
    pub struct PropLang {
        statements: Vec<Statement>
    }

    pub enum Statement {
        #[rust_sitter::prec(3)]
        Assignment(
            Ident,
            #[rust_sitter::leaf(text = "=")] (),
            Expression,
        ),
        // ImpureStatement,
    }

    pub enum Expression {
        MathExpr(MathExpr),
        #[rust_sitter::prec(2)]
        Func(
            Parameters,
            Box<Expression>,
        ),
    }

    pub enum MathExpr {
        Number(
            #[rust_sitter::leaf(pattern = r"\d+", transform = |v| v.parse().unwrap())]
            i32
        ),
        
        #[rust_sitter::prec_left(1)]
        Add(
            Box<MathExpr>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<MathExpr>,
        ),

        #[rust_sitter::prec_left(1)]
        Sub(
            Box<MathExpr>,
            #[rust_sitter::leaf(text = "-")] (),
            Box<MathExpr>,
        ),

        #[rust_sitter::prec_left(2)]
        Mul(
            Box<MathExpr>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<MathExpr>,
        ),

        #[rust_sitter::prec_left(2)]
        Div(
            Box<MathExpr>,
            #[rust_sitter::leaf(text = "/")] (),
            Box<MathExpr>,
        ),

        #[rust_sitter::prec_left(2)]
        Mod(
            Box<MathExpr>,
            #[rust_sitter::leaf(text = "%")] (),
            Box<MathExpr>,
        ),
    }

    #[allow(dead_code)]
    pub struct Parameters {
        #[rust_sitter::leaf(text = "(")] 
        _starting_parenth: (),

        #[rust_sitter::delimited(#[rust_sitter::leaf(text = ",")] ())]
        pub idents: Vec<Ident>,
        
        #[rust_sitter::leaf(text = ")")] 
        _ending_parenth: (),
    }

    #[allow(dead_code)]
    pub struct Ident {
        #[rust_sitter::word]
        #[rust_sitter::leaf(pattern = r"[a-z|A-Z]+", transform = |v| v.to_string())]
        pub ident: String
    }

    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }
}
