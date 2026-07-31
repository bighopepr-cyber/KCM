use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Select,
    From,
    Where,
    And,
    Or,
    Not,
    Limit,
    OrderBy,
    Asc,
    Desc,
    Identifier(String),
    Number(f64),
    StringLit(String),
    Star,
    Comma,
    LeftParen,
    RightParen,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Eof,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(Token::Eof);
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        match self.input.peek() {
            None => Ok(Token::Eof),
            Some('*') => {
                self.input.next();
                Ok(Token::Star)
            }
            Some(',') => {
                self.input.next();
                Ok(Token::Comma)
            }
            Some('(') => {
                self.input.next();
                Ok(Token::LeftParen)
            }
            Some(')') => {
                self.input.next();
                Ok(Token::RightParen)
            }
            Some('=') => {
                self.input.next();
                Ok(Token::Equals)
            }
            Some('!') => {
                self.input.next();
                if self.input.peek() == Some(&'=') {
                    self.input.next();
                    Ok(Token::NotEquals)
                } else {
                    Err("Expected '=' after '!'".to_string())
                }
            }
            Some('<') => {
                self.input.next();
                if self.input.peek() == Some(&'=') {
                    self.input.next();
                    Ok(Token::LessThanOrEqual)
                } else {
                    Ok(Token::LessThan)
                }
            }
            Some('>') => {
                self.input.next();
                if self.input.peek() == Some(&'=') {
                    self.input.next();
                    Ok(Token::GreaterThanOrEqual)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            Some('"') => self.read_string(),
            Some(c) if c.is_ascii_digit() => self.read_number(),
            Some(c) if c.is_ascii_alphabetic() || *c == '_' => self.read_identifier(),
            Some(c) => Err(format!("Unexpected character: '{}'", c)),
        }
    }

    fn read_identifier(&mut self) -> Result<Token, String> {
        let mut ident = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        Ok(match ident.to_lowercase().as_str() {
            "select" => Token::Select,
            "from" => Token::From,
            "where" => Token::Where,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "limit" => Token::Limit,
            "order" | "by" => Token::OrderBy,
            "asc" => Token::Asc,
            "desc" => Token::Desc,
            _ => Token::Identifier(ident),
        })
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut s = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() || c == '.' {
                s.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        s.parse::<f64>()
            .map(Token::Number)
            .map_err(|e| e.to_string())
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.input.next();
        let mut s = String::new();
        while let Some(c) = self.input.next() {
            if c == '"' {
                return Ok(Token::StringLit(s));
            }
            s.push(c);
        }
        Err("Unterminated string".to_string())
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectQuery {
    pub columns: Vec<String>,
    pub from_entity: String,
    pub where_clause: Option<WhereClause>,
    pub order_by: Option<String>,
    pub order_desc: bool,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub enum Condition {
    Equal(String, Value),
    NotEqual(String, Value),
    GreaterThan(String, f64),
    LessThan(String, f64),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        Ok(Parser { tokens, pos: 0 })
    }

    pub fn parse(&mut self) -> Result<SelectQuery, String> {
        self.expect(&Token::Select)?;
        let columns = self.parse_columns()?;
        self.expect(&Token::From)?;
        let from_entity = self.parse_ident()?;
        let where_clause = if self.check(&Token::Where) {
            self.advance();
            Some(self.parse_where()?)
        } else {
            None
        };
        let (order_by, order_desc) = if self.check(&Token::OrderBy) {
            self.advance();
            let col = self.parse_ident()?;
            let desc = self.check(&Token::Desc);
            if desc {
                self.advance();
            }
            (Some(col), desc)
        } else {
            (None, false)
        };
        let limit = if self.check(&Token::Limit) {
            self.advance();
            if let Token::Number(n) = self.advance() {
                Some(n as usize)
            } else {
                return Err("Expected number".to_string());
            }
        } else {
            None
        };
        Ok(SelectQuery {
            columns,
            from_entity,
            where_clause,
            order_by,
            order_desc,
            limit,
        })
    }

    fn parse_columns(&mut self) -> Result<Vec<String>, String> {
        if self.check(&Token::Star) {
            self.advance();
            return Ok(vec!["*".to_string()]);
        }
        let mut cols = vec![self.parse_ident()?];
        while self.check(&Token::Comma) {
            self.advance();
            cols.push(self.parse_ident()?);
        }
        Ok(cols)
    }

    fn parse_where(&mut self) -> Result<WhereClause, String> {
        let mut conditions = Vec::new();
        conditions.push(self.parse_condition()?);
        while self.check(&Token::And) || self.check(&Token::Or) {
            let op_token = self.advance();
            let right = self.parse_condition()?;
            let last = conditions.pop().unwrap();
            let combined = match op_token {
                Token::And => Condition::And(Box::new(last), Box::new(right)),
                Token::Or => Condition::Or(Box::new(last), Box::new(right)),
                _ => unreachable!(),
            };
            conditions.push(combined);
        }
        Ok(WhereClause { conditions })
    }

    fn parse_condition(&mut self) -> Result<Condition, String> {
        let left = self.parse_ident()?;
        let op = self.advance();
        match op {
            Token::Equals => match self.advance() {
                Token::Number(n) => Ok(Condition::Equal(left, Value::Number(n))),
                Token::Identifier(s) => Ok(Condition::Equal(left, Value::String(s))),
                Token::StringLit(s) => Ok(Condition::Equal(left, Value::String(s))),
                t => Err(format!("Expected value after '=', got {:?}", t)),
            },
            Token::GreaterThan => {
                let n = self.parse_number()?;
                Ok(Condition::GreaterThan(left, n))
            }
            Token::LessThan => {
                let n = self.parse_number()?;
                Ok(Condition::LessThan(left, n))
            }
            _ => Err(format!("Expected operator, got {:?}", op)),
        }
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        match self.advance() {
            Token::Identifier(s) => Ok(s),
            t => Err(format!("Expected identifier, got {:?}", t)),
        }
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        match self.advance() {
            Token::Number(n) => Ok(n),
            t => Err(format!("Expected number, got {:?}", t)),
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.peek()))
        }
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == *expected
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let t = self.peek();
        self.pos += 1;
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_select() {
        let mut lexer = Lexer::new("SELECT * FROM facts WHERE subject = 1");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Select);
        assert_eq!(tokens[1], Token::Star);
        assert_eq!(tokens[2], Token::From);
        assert_eq!(tokens[4], Token::Where);
    }

    #[test]
    fn test_parser_basic() {
        let mut parser =
            Parser::new("SELECT subject, object FROM facts WHERE predicate = 0 LIMIT 100").unwrap();
        let query = parser.parse().unwrap();
        assert_eq!(query.columns, vec!["subject", "object"]);
        assert_eq!(query.from_entity, "facts");
        assert_eq!(query.limit, Some(100));
    }

    #[test]
    fn test_parser_string_literal() {
        let mut parser = Parser::new(r#"SELECT * FROM facts WHERE name = "test""#).unwrap();
        let query = parser.parse().unwrap();
        assert!(query.where_clause.is_some());
    }
}
