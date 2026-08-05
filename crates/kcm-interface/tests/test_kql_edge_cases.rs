use kcm_interface::kql_parser::{KqlError, Lexer, Parser, Token};

#[test]
fn test_lexer_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(format!("{:?}", tokens[0]), "Eof");
}

#[test]
fn test_lexer_whitespace_only() {
    let mut lexer = Lexer::new("   \t\n  ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(format!("{:?}", tokens[0]), "Eof");
}

#[test]
fn test_lexer_unterminated_string() {
    let mut lexer = Lexer::new(r#"SELECT * FROM facts WHERE name = "test"#);
    let result = lexer.tokenize();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), KqlError::UnterminatedString));
}

#[test]
fn test_lexer_special_characters_in_string() {
    let mut lexer = Lexer::new(r#"SELECT * FROM facts WHERE id = 42 AND name = "hello world""#);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(format!("{:?}", tokens[0]), "Select");
    assert_eq!(tokens.len(), 13);
}

#[test]
fn test_lexer_all_operators() {
    let mut lexer = Lexer::new("= != < > <= >= * ( ) ,");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 11);
    assert_eq!(format!("{:?}", tokens[0]), "Equals");
    assert_eq!(format!("{:?}", tokens[1]), "NotEquals");
    assert_eq!(format!("{:?}", tokens[2]), "LessThan");
    assert_eq!(format!("{:?}", tokens[3]), "GreaterThan");
    assert_eq!(format!("{:?}", tokens[4]), "LessThanOrEqual");
    assert_eq!(format!("{:?}", tokens[5]), "GreaterThanOrEqual");
    assert_eq!(format!("{:?}", tokens[6]), "Star");
    assert_eq!(format!("{:?}", tokens[7]), "LeftParen");
    assert_eq!(format!("{:?}", tokens[8]), "RightParen");
    assert_eq!(format!("{:?}", tokens[9]), "Comma");
    assert_eq!(format!("{:?}", tokens[10]), "Eof");
}

#[test]
fn test_lexer_all_keywords() {
    let mut lexer =
        Lexer::new("SELECT FROM WHERE AND OR NOT LIMIT ORDER BY ASC DESC JOIN ON INFER");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 15);
    assert_eq!(format!("{:?}", tokens[0]), "Select");
    assert_eq!(format!("{:?}", tokens[1]), "From");
    assert_eq!(format!("{:?}", tokens[2]), "Where");
    assert_eq!(format!("{:?}", tokens[3]), "And");
    assert_eq!(format!("{:?}", tokens[4]), "Or");
    assert_eq!(format!("{:?}", tokens[5]), "Not");
    assert_eq!(format!("{:?}", tokens[6]), "Limit");
    assert_eq!(format!("{:?}", tokens[7]), "Order");
    assert_eq!(format!("{:?}", tokens[8]), "By");
    assert_eq!(format!("{:?}", tokens[9]), "Asc");
    assert_eq!(format!("{:?}", tokens[10]), "Desc");
    assert_eq!(format!("{:?}", tokens[11]), "Join");
    assert_eq!(format!("{:?}", tokens[12]), "On");
    assert_eq!(format!("{:?}", tokens[13]), "Infer");
    assert_eq!(format!("{:?}", tokens[14]), "Eof");
}

#[test]
fn test_lexer_large_number() {
    let mut lexer = Lexer::new("WHERE id = 999999999");
    let tokens = lexer.tokenize().unwrap();
    let mut found_number = false;
    for token in &tokens {
        if let Token::Number(n) = token {
            assert_eq!(*n, 999_999_999.0);
            found_number = true;
        }
    }
    assert!(found_number);
}

#[test]
fn test_lexer_decimal_number() {
    let mut lexer = Lexer::new("WHERE confidence = 0.75");
    let tokens = lexer.tokenize().unwrap();
    let mut found_number = false;
    for token in &tokens {
        if let Token::Number(n) = token {
            assert_eq!(*n, 0.75);
            found_number = true;
        }
    }
    assert!(found_number);
}

#[test]
fn test_lexer_exclamation_without_equals() {
    let mut lexer = Lexer::new("!");
    let result = lexer.tokenize();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        KqlError::UnexpectedCharacter('!')
    ));
}

#[test]
fn test_lexer_unexpected_character() {
    let mut lexer = Lexer::new("@");
    let result = lexer.tokenize();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        KqlError::UnexpectedCharacter('@')
    ));
}

#[test]
fn test_lexer_identifier_with_underscores() {
    let mut lexer = Lexer::new("my_table_name");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    if let Token::Identifier(name) = &tokens[0] {
        assert_eq!(name, "my_table_name");
    } else {
        panic!("Expected Identifier token");
    }
}

#[test]
fn test_lexer_identifier_starting_with_underscore() {
    let mut lexer = Lexer::new("_private_col");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    if let Token::Identifier(name) = &tokens[0] {
        assert_eq!(name, "_private_col");
    } else {
        panic!("Expected Identifier token");
    }
}

#[test]
fn test_lexer_multiple_strings() {
    let mut lexer = Lexer::new(r#"WHERE a = "foo" AND b = "bar""#);
    let tokens = lexer.tokenize().unwrap();
    let mut strings = Vec::new();
    for token in &tokens {
        if let Token::StringLit(s) = token {
            strings.push(s.clone());
        }
    }
    assert_eq!(strings, vec!["foo".to_string(), "bar".to_string()]);
}

#[test]
fn test_lexer_keyword_case_insensitivity() {
    let mut lexer = Lexer::new("select Select SELECT sElEcT");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 5);
    for token in &tokens[..tokens.len() - 1] {
        assert_eq!(format!("{:?}", token), "Select");
    }
    assert_eq!(format!("{:?}", tokens.last().unwrap()), "Eof");
}

#[test]
fn test_lexer_adjacent_operators() {
    let mut lexer = Lexer::new("=!=<><=>=");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 7);
    assert_eq!(format!("{:?}", tokens[0]), "Equals");
    assert_eq!(format!("{:?}", tokens[1]), "NotEquals");
    assert_eq!(format!("{:?}", tokens[2]), "LessThan");
    assert_eq!(format!("{:?}", tokens[3]), "GreaterThan");
    assert_eq!(format!("{:?}", tokens[4]), "LessThanOrEqual");
    assert_eq!(format!("{:?}", tokens[5]), "GreaterThanOrEqual");
    assert_eq!(format!("{:?}", tokens[6]), "Eof");
}

#[test]
fn test_parser_basic_select_star() {
    let mut parser = Parser::new("SELECT * FROM facts").unwrap();
    let query = parser.parse().unwrap();
    assert_eq!(query.columns, vec!["*"]);
    assert_eq!(query.from_entity, "facts");
    assert!(query.where_clause.is_none());
    assert!(query.limit.is_none());
}

#[test]
fn test_parser_select_columns() {
    let mut parser =
        Parser::new("SELECT subject, object FROM facts WHERE predicate = 0 LIMIT 100").unwrap();
    let query = parser.parse().unwrap();
    assert_eq!(query.columns, vec!["subject", "object"]);
    assert_eq!(query.from_entity, "facts");
    assert_eq!(query.limit, Some(100));
}

#[test]
fn test_parser_string_literal_condition() {
    let mut parser = Parser::new(r#"SELECT * FROM facts WHERE name = "test""#).unwrap();
    let query = parser.parse().unwrap();
    assert!(query.where_clause.is_some());
}

#[test]
fn test_parser_numeric_condition() {
    let mut parser = Parser::new("SELECT * FROM facts WHERE id = 42").unwrap();
    let query = parser.parse().unwrap();
    let wc = query.where_clause.unwrap();
    assert_eq!(wc.conditions.len(), 1);
}

#[test]
fn test_parser_order_by_asc() {
    let mut parser = Parser::new("SELECT * FROM facts ORDER BY confidence ASC").unwrap();
    let query = parser.parse().unwrap();
    let ob = query.order_by.unwrap();
    assert_eq!(ob.column, "confidence");
    assert!(matches!(
        ob.direction,
        kcm_interface::kql_parser::OrderByDirection::Asc
    ));
}

#[test]
fn test_parser_order_by_desc() {
    let mut parser = Parser::new("SELECT * FROM facts ORDER BY confidence DESC").unwrap();
    let query = parser.parse().unwrap();
    let ob = query.order_by.unwrap();
    assert_eq!(ob.column, "confidence");
    assert!(matches!(
        ob.direction,
        kcm_interface::kql_parser::OrderByDirection::Desc
    ));
}

#[test]
fn test_parser_infer_keyword() {
    let mut parser = Parser::new("SELECT * FROM facts INFER").unwrap();
    let query = parser.parse().unwrap();
    assert_eq!(query.from_entity, "facts");
}

#[test]
fn test_parser_invalid_start() {
    let mut parser = Parser::new("FROM facts").unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_lexer_number_with_multiple_dots() {
    let mut lexer = Lexer::new("1.2.3");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_lexer_empty_string() {
    let mut lexer = Lexer::new(r#"WHERE name = """#);
    let tokens = lexer.tokenize().unwrap();
    let mut found_empty_string = false;
    for token in &tokens {
        if let Token::StringLit(s) = token {
            assert!(s.is_empty());
            found_empty_string = true;
        }
    }
    assert!(found_empty_string);
}

#[test]
fn test_parser_and_condition() {
    let mut parser = Parser::new("SELECT * FROM facts WHERE subject = 1 AND object = 2").unwrap();
    let query = parser.parse().unwrap();
    let wc = query.where_clause.unwrap();
    assert_eq!(wc.conditions.len(), 2);
}

#[test]
fn test_parser_or_condition() {
    let mut parser = Parser::new("SELECT * FROM facts WHERE subject = 1 OR subject = 2").unwrap();
    let query = parser.parse().unwrap();
    let wc = query.where_clause.unwrap();
    assert_eq!(wc.conditions.len(), 2);
}

#[test]
fn test_parser_where_greater_than_or_equal() {
    let mut parser = Parser::new("SELECT * FROM facts WHERE confidence >= 0.5").unwrap();
    let query = parser.parse().unwrap();
    let wc = query.where_clause.unwrap();
    assert_eq!(wc.conditions.len(), 1);
    assert!(matches!(
        wc.conditions[0],
        kcm_interface::kql_parser::Condition::GreaterThanOrEqual(_, _)
    ));
}

#[test]
fn test_parser_where_less_than_or_equal() {
    let mut parser = Parser::new("SELECT * FROM facts WHERE confidence <= 0.8").unwrap();
    let query = parser.parse().unwrap();
    let wc = query.where_clause.unwrap();
    assert_eq!(wc.conditions.len(), 1);
    assert!(matches!(
        wc.conditions[0],
        kcm_interface::kql_parser::Condition::LessThanOrEqual(_, _)
    ));
}

#[test]
fn test_parser_where_not() {
    let mut parser = Parser::new("SELECT * FROM facts WHERE NOT subject = 1").unwrap();
    let query = parser.parse().unwrap();
    let wc = query.where_clause.unwrap();
    assert_eq!(wc.conditions.len(), 1);
    assert!(matches!(
        wc.conditions[0],
        kcm_interface::kql_parser::Condition::Not(_)
    ));
}
