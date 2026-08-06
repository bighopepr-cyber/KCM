use kcm_core::types::KcmError;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum KqlError {
    UnexpectedCharacter(char),
    UnexpectedToken(String),
    UnterminatedString,
    ExpectedIdentifier,
    ExpectedNumber,
    ExpectedComparisonOperator,
    UnexpectedEof,
}

impl fmt::Display for KqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KqlError::UnexpectedCharacter(c) => write!(f, "Unexpected character: '{}'", c),
            KqlError::UnexpectedToken(t) => write!(f, "Unexpected token: {}", t),
            KqlError::UnterminatedString => write!(f, "Unterminated string literal"),
            KqlError::ExpectedIdentifier => write!(f, "Expected identifier"),
            KqlError::ExpectedNumber => write!(f, "Expected number"),
            KqlError::ExpectedComparisonOperator => write!(f, "Expected comparison operator"),
            KqlError::UnexpectedEof => write!(f, "Unexpected end of input"),
        }
    }
}

impl std::error::Error for KqlError {}

impl From<KqlError> for KcmError {
    fn from(err: KqlError) -> Self {
        KcmError::InvalidArgument(err.to_string())
    }
}

pub type KqlResult<T> = Result<T, KqlError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Select,
    From,
    Where,
    And,
    Or,
    Not,
    Limit,
    Order,
    By,
    Asc,
    Desc,
    Join,
    On,
    Infer,
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

    pub fn tokenize(&mut self) -> KqlResult<Vec<Token>> {
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

    pub fn next_token(&mut self) -> KqlResult<Token> {
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
                    Err(KqlError::UnexpectedCharacter('!'))
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
            Some(c) => Err(KqlError::UnexpectedCharacter(*c)),
        }
    }

    fn read_identifier(&mut self) -> KqlResult<Token> {
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
            "order" => Token::Order,
            "by" => Token::By,
            "asc" => Token::Asc,
            "desc" => Token::Desc,
            "join" => Token::Join,
            "on" => Token::On,
            "infer" => Token::Infer,
            _ => Token::Identifier(ident),
        })
    }

    fn read_number(&mut self) -> KqlResult<Token> {
        let mut num_str = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        num_str
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| KqlError::ExpectedNumber)
    }

    fn read_string(&mut self) -> KqlResult<Token> {
        self.input.next();
        let mut string = String::new();
        for c in self.input.by_ref() {
            if c == '"' {
                return Ok(Token::StringLit(string));
            }
            string.push(c);
        }
        Err(KqlError::UnterminatedString)
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
    pub join: Option<JoinClause>,
    pub order_by: Option<OrderByClause>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub enum Condition {
    Equal(String, String),
    GreaterThan(String, f64),
    LessThan(String, f64),
    GreaterThanOrEqual(String, f64),
    LessThanOrEqual(String, f64),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

#[derive(Debug, Clone)]
pub struct JoinClause {
    pub entity: String,
    pub on: (String, String),
}

#[derive(Debug, Clone)]
pub enum OrderByDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct OrderByClause {
    pub column: String,
    pub direction: OrderByDirection,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> KqlResult<Self> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        Ok(Parser { tokens, pos: 0 })
    }

    pub fn parse(&mut self) -> KqlResult<SelectQuery> {
        self.expect(Token::Select)?;
        let columns = self.parse_column_list()?;
        self.expect(Token::From)?;
        let from_entity = self.parse_identifier()?;

        let where_clause = if self.peek() == &Token::Where {
            self.next();
            Some(self.parse_where_clause()?)
        } else {
            None
        };

        let join = if self.peek() == &Token::Join {
            self.next();
            Some(self.parse_join_clause()?)
        } else {
            None
        };

        let order_by = if self.peek() == &Token::Order {
            self.next();
            self.expect(Token::By)?;
            Some(self.parse_order_by_clause()?)
        } else {
            None
        };

        let limit = if self.peek() == &Token::Limit {
            self.next();
            if let Token::Number(n) = self.next() {
                Some(n as usize)
            } else {
                return Err(KqlError::ExpectedNumber);
            }
        } else {
            None
        };

        Ok(SelectQuery {
            columns,
            from_entity,
            where_clause,
            join,
            order_by,
            limit,
        })
    }

    fn parse_column_list(&mut self) -> KqlResult<Vec<String>> {
        let mut columns = Vec::new();
        if self.peek() == &Token::Star {
            self.next();
            columns.push("*".to_string());
        } else {
            columns.push(self.parse_identifier()?);
            while self.peek() == &Token::Comma {
                self.next();
                columns.push(self.parse_identifier()?);
            }
        }
        Ok(columns)
    }

    fn parse_where_clause(&mut self) -> KqlResult<WhereClause> {
        let mut conditions = Vec::new();
        loop {
            let left = if self.peek() == &Token::Not {
                self.next();
                let inner = self.parse_single_condition()?;
                Condition::Not(Box::new(inner))
            } else {
                self.parse_single_condition()?
            };
            conditions.push(left);
            if self.peek() != &Token::And && self.peek() != &Token::Or {
                break;
            }
            self.next();
        }
        Ok(WhereClause { conditions })
    }

    fn parse_single_condition(&mut self) -> KqlResult<Condition> {
        let left = self.parse_identifier()?;
        let op_token = self.next();
        match op_token {
            Token::Equals => {
                let right = match self.peek() {
                    Token::Number(_) => self.parse_number()?.to_string(),
                    Token::StringLit(_) => {
                        if let Token::StringLit(s) = self.next() {
                            s
                        } else {
                            unreachable!()
                        }
                    }
                    _ => self.parse_identifier()?,
                };
                Ok(Condition::Equal(left, right))
            }
            Token::GreaterThan => {
                let right = self.parse_number()?;
                Ok(Condition::GreaterThan(left, right))
            }
            Token::LessThan => {
                let right = self.parse_number()?;
                Ok(Condition::LessThan(left, right))
            }
            Token::GreaterThanOrEqual => {
                let right = self.parse_number()?;
                Ok(Condition::GreaterThanOrEqual(left, right))
            }
            Token::LessThanOrEqual => {
                let right = self.parse_number()?;
                Ok(Condition::LessThanOrEqual(left, right))
            }
            _ => Err(KqlError::ExpectedComparisonOperator),
        }
    }

    fn parse_join_clause(&mut self) -> KqlResult<JoinClause> {
        let entity = self.parse_identifier()?;
        self.expect(Token::On)?;
        let left_col = self.parse_identifier()?;
        self.expect(Token::Equals)?;
        let right_col = self.parse_identifier()?;
        Ok(JoinClause {
            entity,
            on: (left_col, right_col),
        })
    }

    fn parse_order_by_clause(&mut self) -> KqlResult<OrderByClause> {
        let column = self.parse_identifier()?;
        let direction = if self.peek() == &Token::Desc {
            self.next();
            OrderByDirection::Desc
        } else {
            OrderByDirection::Asc
        };
        Ok(OrderByClause { column, direction })
    }

    fn parse_identifier(&mut self) -> KqlResult<String> {
        match self.next() {
            Token::Identifier(name) => Ok(name),
            _ => Err(KqlError::ExpectedIdentifier),
        }
    }

    fn parse_number(&mut self) -> KqlResult<f64> {
        match self.next() {
            Token::Number(n) => Ok(n),
            _ => Err(KqlError::ExpectedNumber),
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn next(&mut self) -> Token {
        let token = self.peek().clone();
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: Token) -> KqlResult<()> {
        if self.peek() == &expected {
            self.next();
            Ok(())
        } else {
            Err(KqlError::UnexpectedToken(format!("{:?}", self.peek())))
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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

    #[test]
    fn test_kql_error_conversion() {
        let err = KqlError::UnexpectedCharacter('!');
        let kcm_err: KcmError = err.into();
        assert!(matches!(kcm_err, KcmError::InvalidArgument(_)));
    }

    #[test]
    fn test_lexer_unterminated_string() {
        let mut lexer = Lexer::new(r#"SELECT * FROM facts WHERE name = "test"#);
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_lexer_unexpected_character() {
        let mut lexer = Lexer::new("SELECT * FROM facts WHERE @invalid");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }
}
