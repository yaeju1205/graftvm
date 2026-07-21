/// Token types for the Parlance language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Integer literal: `42`, `-3`
    Int(i64),
    /// Float literal: `3.14`, `-1.0`
    Float(f64),
    /// String literal: `"hello"`
    String(String),
    /// Boolean: `true`, `false`
    Bool(bool),
    /// Identifier: `x`, `add`, `+`, `infix`, `def`
    Ident(String),
    /// Opening parenthesis `(`
    LParen,
    /// Closing parenthesis `)`
    RParen,
    /// End of input
    Eof,
}

/// Convert source text into a sequence of tokens.
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace
            c if c.is_whitespace() => {
                chars.next();
            }
            // Line comments: `;;` to end of line
            ';' => {
                chars.next();
                if chars.peek() == Some(&';') {
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        chars.next();
                    }
                }
            }
            // Parentheses
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            // String literals
            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '"' => {
                            chars.next();
                            break;
                        }
                        '\\' => {
                            chars.next();
                            if let Some(esc) = chars.next() {
                                match esc {
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    '"' => s.push('"'),
                                    '\\' => s.push('\\'),
                                    _ => s.push(esc),
                                }
                            }
                        }
                        _ => {
                            s.push(c);
                            chars.next();
                        }
                    }
                }
                tokens.push(Token::String(s));
            }
            // Numbers or identifiers starting with digit
            c if c.is_ascii_digit() || (c == '-' && chars.clone().nth(1).map_or(false, |n| n.is_ascii_digit())) => {
                let mut num = String::new();
                if c == '-' {
                    num.push('-');
                    chars.next();
                }
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' || c == '_' {
                        if c != '_' {
                            num.push(c);
                        }
                        chars.next();
                    } else {
                        break;
                    }
                }
                if num.contains('.') {
                    tokens.push(Token::Float(num.parse().unwrap_or(0.0)));
                } else {
                    tokens.push(Token::Int(num.parse().unwrap_or(0)));
                }
            }
            // Identifiers (including operator symbols)
            _ => {
                let mut ident = String::new();
                // Operators: + - * / % < > = ! & | ^ ~
                if "+-*/%<>=!&|^~".contains(ch) {
                    while let Some(&c) = chars.peek() {
                        if "+-*/%<>=!&|^~".contains(c) {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                } else {
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '\'' || c == '?' {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                match ident.as_str() {
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
        }
    }

    tokens.push(Token::Eof);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tokens() {
        let t = lex("(+ 1 2)");
        assert_eq!(t.len(), 6); // LParen, Ident(+), Int(1), Int(2), RParen, Eof
        assert_eq!(t[0], Token::LParen);
        assert_eq!(t[1], Token::Ident("+".into()));
        assert_eq!(t[2], Token::Int(1));
        assert_eq!(t[3], Token::Int(2));
        assert_eq!(t[4], Token::RParen);
    }

    #[test]
    fn comment() {
        let t = lex(";; comment\n(+ 1 2)");
        assert_eq!(t.len(), 6);
        assert_eq!(t[1], Token::Ident("+".into()));
    }

    #[test]
    fn negative_number() {
        let t = lex("-42");
        assert_eq!(t[0], Token::Int(-42));
    }

    #[test]
    fn bool() {
        let t = lex("true false");
        assert_eq!(t[0], Token::Bool(true));
        assert_eq!(t[1], Token::Bool(false));
        assert_eq!(t[2], Token::Eof);
    }

    #[test]
    fn string_literal() {
        let t = lex(r#""hello world""#);
        assert_eq!(t[0], Token::String("hello world".into()));
    }
}
