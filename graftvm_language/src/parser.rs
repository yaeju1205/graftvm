use crate::lexer::Token;

/// A named infix operator declaration.
#[derive(Debug, Clone)]
pub struct InfixDecl {
    pub symbol: String,
    pub precedence: u32,
}

/// A function definition.
#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

/// An expression in the Parlance language.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal
    Int(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Boolean literal
    Bool(bool),
    /// Variable reference
    Var(String),
    /// Function application: `(f x y ...)`
    Apply {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    /// Lambda abstraction: `(lambda (x y) body)`
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    /// Let binding: `(let ((x 1) (y 2)) body)`
    Let {
        bindings: Vec<(String, Expr)>,
        body: Box<Expr>,
    },
    /// Conditional: `(if cond then else)`
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
}

/// Parse a complete source into a list of top-level expressions and declarations.
pub fn parse(tokens: &[Token]) -> Result<ParseResult, String> {
    let mut pos = 0;
    let mut funcs = Vec::new();
    let mut infixes = Vec::new();
    let mut exprs = Vec::new();

    while tokens[pos] != Token::Eof {
        if tokens[pos] == Token::LParen {
            if pos + 1 < tokens.len() {
                match &tokens[pos + 1] {
                    Token::Ident(s) if s == "def" => {
                        let (f, new_pos) = parse_def(tokens, pos)?;
                        funcs.push(f);
                        pos = new_pos;
                        continue;
                    }
                    Token::Ident(s) if s == "infix" => {
                        let (inf, new_pos) = parse_infix(tokens, pos)?;
                        infixes.push(inf);
                        pos = new_pos;
                        continue;
                    }
                    _ => {}
                }
            }
        }

        let (expr, new_pos) = parse_expr(tokens, pos, &infixes)?;
        exprs.push(expr);
        pos = new_pos;
    }

    Ok(ParseResult { funcs, infixes, exprs })
}

/// The result of parsing a source file.
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub funcs: Vec<FuncDef>,
    pub infixes: Vec<InfixDecl>,
    pub exprs: Vec<Expr>,
}

fn parse_def(tokens: &[Token], start: usize) -> Result<(FuncDef, usize), String> {
    let mut pos = start + 2;
    let name = match &tokens[pos] {
        Token::Ident(s) => s.clone(),
        _ => return Err("expected function name after def".into()),
    };
    pos += 1;

    let mut params = Vec::new();
    if tokens[pos] == Token::LParen {
        pos += 1;
        while tokens[pos] != Token::RParen && tokens[pos] != Token::Eof {
            match &tokens[pos] {
                Token::Ident(s) => params.push(s.clone()),
                _ => return Err("expected parameter name".into()),
            }
            pos += 1;
        }
        pos += 1;
    } else {
        while let Token::Ident(s) = &tokens[pos] {
            if s.chars().next().map_or(false, |c| c.is_lowercase()) {
                params.push(s.clone());
                pos += 1;
            } else {
                break;
            }
        }
    }

    let (body, new_pos) = parse_expr(tokens, pos, &[])?;
    if tokens[new_pos] != Token::RParen {
        return Err("expected ')' after function body".into());
    }
    Ok((FuncDef { name, params, body: Box::new(body) }, new_pos + 1))
}

fn parse_infix(tokens: &[Token], start: usize) -> Result<(InfixDecl, usize), String> {
    let mut pos = start + 2;
    let symbol = match &tokens[pos] {
        Token::Ident(s) => s.clone(),
        _ => return Err("expected operator symbol after infix".into()),
    };
    pos += 1;
    let precedence = match &tokens[pos] {
        Token::Int(n) => *n as u32,
        _ => return Err("expected precedence number".into()),
    };
    pos += 1;
    if tokens[pos] != Token::RParen {
        return Err("expected ')' after infix declaration".into());
    }
    Ok((InfixDecl { symbol, precedence }, pos + 1))
}

fn parse_expr(tokens: &[Token], start: usize, infixes: &[InfixDecl]) -> Result<(Expr, usize), String> {
    if start >= tokens.len() {
        return Err("unexpected end of input".into());
    }
    match &tokens[start] {
        Token::LParen => {
            if start + 1 >= tokens.len() {
                return Err("empty parentheses".into());
            }
            match &tokens[start + 1] {
                Token::Ident(s) if s == "lambda" => parse_lambda(tokens, start),
                Token::Ident(s) if s == "let" => parse_let(tokens, start),
                Token::Ident(s) if s == "if" => parse_if(tokens, start),
                _ => parse_application(tokens, start, infixes),
            }
        }
        Token::Int(n) => Ok((Expr::Int(*n), start + 1)),
        Token::Float(n) => Ok((Expr::Float(*n), start + 1)),
        Token::String(s) => Ok((Expr::String(s.clone()), start + 1)),
        Token::Bool(b) => Ok((Expr::Bool(*b), start + 1)),
        Token::Ident(s) => Ok((Expr::Var(s.clone()), start + 1)),
        Token::RParen => Err("unexpected ')'".into()),
        Token::Eof => Err("unexpected end of input".into()),
    }
}

fn parse_lambda(tokens: &[Token], start: usize) -> Result<(Expr, usize), String> {
    let mut pos = start + 2;
    let mut params = Vec::new();
    if tokens[pos] == Token::LParen {
        pos += 1;
        while tokens[pos] != Token::RParen && tokens[pos] != Token::Eof {
            match &tokens[pos] {
                Token::Ident(s) => params.push(s.clone()),
                _ => return Err("expected parameter name in lambda".into()),
            }
            pos += 1;
        }
        pos += 1;
    }
    let (body, new_pos) = parse_expr(tokens, pos, &[])?;
    if tokens[new_pos] != Token::RParen {
        return Err("expected ')' after lambda body".into());
    }
    Ok((Expr::Lambda { params, body: Box::new(body) }, new_pos + 1))
}

fn parse_let(tokens: &[Token], start: usize) -> Result<(Expr, usize), String> {
    let mut pos = start + 2;
    let mut bindings = Vec::new();
    if tokens[pos] == Token::LParen {
        pos += 1;
        while tokens[pos] != Token::RParen && tokens[pos] != Token::Eof {
            if tokens[pos] != Token::LParen {
                return Err("expected (var val) in let".into());
            }
            pos += 1;
            let name = match &tokens[pos] {
                Token::Ident(s) => s.clone(),
                _ => return Err("expected variable name in let".into()),
            };
            pos += 1;
            let (val_expr, new_pos) = parse_expr(tokens, pos, &[])?;
            pos = new_pos;
            if tokens[pos] != Token::RParen {
                return Err("expected ')' after let binding value".into());
            }
            pos += 1;
            bindings.push((name, val_expr));
        }
        pos += 1;
    } else {
        return Err("expected (bindings) in let".into());
    }
    let (body, new_pos) = parse_expr(tokens, pos, &[])?;
    if tokens[new_pos] != Token::RParen {
        return Err("expected ')' after let body".into());
    }
    Ok((Expr::Let { bindings, body: Box::new(body) }, new_pos + 1))
}

fn parse_if(tokens: &[Token], start: usize) -> Result<(Expr, usize), String> {
    let mut pos = start + 2;
    let (cond, new_pos) = parse_expr(tokens, pos, &[])?;
    pos = new_pos;
    let (then, new_pos) = parse_expr(tokens, pos, &[])?;
    pos = new_pos;
    let (else_, new_pos) = parse_expr(tokens, pos, &[])?;
    pos = new_pos;
    if tokens[pos] != Token::RParen {
        return Err("expected ')' after if expression".into());
    }
    Ok((Expr::If {
        cond: Box::new(cond),
        then: Box::new(then),
        else_: Box::new(else_),
    }, pos + 1))
}

fn parse_application(tokens: &[Token], start: usize, _infixes: &[InfixDecl]) -> Result<(Expr, usize), String> {
    let mut pos = start + 1;
    let (func, new_pos) = parse_expr(tokens, pos, _infixes)?;
    pos = new_pos;
    let mut args = Vec::new();
    while tokens[pos] != Token::RParen && tokens[pos] != Token::Eof {
        let (arg, new_pos) = parse_expr(tokens, pos, _infixes)?;
        args.push(arg);
        pos = new_pos;
    }
    if tokens[pos] != Token::RParen {
        return Err("expected ')'".into());
    }
    Ok((Expr::Apply {
        func: Box::new(func),
        args,
    }, pos + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_atom() {
        let tokens = crate::lexer::lex("42 true");
        let res = parse(&tokens).unwrap();
        assert_eq!(res.exprs.len(), 2);
    }

    #[test]
    fn parse_apply() {
        let tokens = crate::lexer::lex("(+ 1 2)");
        let res = parse(&tokens).unwrap();
        assert_eq!(res.exprs.len(), 1);
        match &res.exprs[0] {
            Expr::Apply { func, args } => {
                assert_eq!(args.len(), 2);
                match func.as_ref() {
                    Expr::Var(s) => assert_eq!(s, "+"),
                    _ => panic!("expected Var"),
                }
            }
            _ => panic!("expected Apply"),
        }
    }

    #[test]
    fn parse_def_and_apply() {
        let tokens = crate::lexer::lex("(def add x y (+ x y)) (+ 1 2)");
        let res = parse(&tokens).unwrap();
        assert_eq!(res.funcs.len(), 1);
        assert_eq!(res.funcs[0].name, "add");
        assert_eq!(res.exprs.len(), 1);
    }

    #[test]
    fn parse_infix_decl() {
        let tokens = crate::lexer::lex("(infix + 3)");
        let res = parse(&tokens).unwrap();
        assert_eq!(res.infixes.len(), 1);
        assert_eq!(res.infixes[0].symbol, "+");
        assert_eq!(res.infixes[0].precedence, 3);
    }

    #[test]
    fn parse_if() {
        let tokens = crate::lexer::lex("(if true 1 2)");
        let res = parse(&tokens).unwrap();
        match &res.exprs[0] {
            Expr::If { cond, then, else_ } => {
                assert!(matches!(cond.as_ref(), Expr::Bool(true)));
                assert!(matches!(then.as_ref(), Expr::Int(1)));
                assert!(matches!(else_.as_ref(), Expr::Int(2)));
            }
            _ => panic!("expected If"),
        }
    }

    #[test]
    fn parse_let() {
        let tokens = crate::lexer::lex("(let ((x 1)) x)");
        let res = parse(&tokens).unwrap();
        match &res.exprs[0] {
            Expr::Let { bindings, body } => {
                assert_eq!(bindings.len(), 1);
                assert_eq!(bindings[0].0, "x");
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn parse_lambda() {
        let tokens = crate::lexer::lex("(lambda (x) x)");
        let res = parse(&tokens).unwrap();
        match &res.exprs[0] {
            Expr::Lambda { params, body } => {
                assert_eq!(params.len(), 1);
                assert!(matches!(body.as_ref(), Expr::Var(s) if s == "x"));
            }
            _ => panic!("expected Lambda"),
        }
    }
}
