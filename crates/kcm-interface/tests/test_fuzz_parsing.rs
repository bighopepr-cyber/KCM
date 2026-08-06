#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_interface::kql_parser::{Lexer, Parser};

#[test]
fn test_fuzz_select_variants() {
    let queries = vec![
        "SELECT * FROM facts",
        "SELECT subject FROM facts",
        "SELECT subject, object FROM facts",
        "SELECT subject, predicate, object FROM facts",
        "SELECT * FROM facts WHERE subject = 1",
        "SELECT * FROM facts WHERE predicate = 0 AND object = 5",
        "SELECT * FROM facts WHERE subject = 1 OR subject = 2",
        "SELECT * FROM facts WHERE subject = 1 AND predicate = 0 AND confidence = 0.3",
        "SELECT * FROM facts LIMIT 10",
        "SELECT * FROM facts WHERE subject = 1 LIMIT 100",
    ];
    for q in queries {
        let mut parser = Parser::new(q).unwrap();
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse: {}\nError: {:?}",
            q,
            result.err()
        );
    }
}

#[test]
fn test_fuzz_invalid_syntax() {
    let invalid = vec![
        "",
        "FROM facts SELECT *",
        "SELECT * WHERE subject = 1",
        "SELECT",
        "SELECT * FROM",
        "SELECT * FROM facts WHERE",
    ];
    for q in invalid {
        let result = Parser::new(q).and_then(|mut p| p.parse());
        assert!(result.is_err(), "Should fail: {}", q);
    }
}

#[test]
fn test_fuzz_lexer_tokens() {
    let inputs = vec![
        "SELECT * FROM facts WHERE subject = 1 AND object = 0.5",
        "SELECT a, b, c FROM table WHERE x = 1 OR y = 2 ORDER BY z LIMIT 100",
        "SELECT * FROM facts JOIN other ON subject = id INFER rule_1",
        "SELECT * FROM facts WHERE subject = 0 AND predicate = 0 AND object = 0",
    ];
    for input in inputs {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert!(
            tokens.len() > 5,
            "Should produce many tokens for: {}",
            input
        );
    }
}

#[test]
fn test_fuzz_long_identifiers() {
    let long_ident = "a".repeat(1000);
    let query = format!("SELECT {} FROM facts", long_ident);
    let mut lexer = Lexer::new(&query);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() >= 4);
}

#[test]
fn test_fuzz_string_literal_empty_body() {
    let mut lexer = Lexer::new(r#"SELECT * FROM facts WHERE name = "hello""#);
    let result = lexer.tokenize();
    assert!(result.is_ok());
}

#[test]
fn test_fuzz_unterminated_string_literal() {
    let mut lexer = Lexer::new(r#"SELECT * FROM facts WHERE name = "hello"#);
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_fuzz_numeric_boundaries() {
    let queries = vec![
        "SELECT * FROM facts WHERE confidence = 0.0",
        "SELECT * FROM facts WHERE confidence = 1.0",
        "SELECT * FROM facts WHERE confidence = 0.5",
        "SELECT * FROM facts WHERE subject = 0",
        "SELECT * FROM facts WHERE subject = 4294967295",
    ];
    for q in queries {
        let mut parser = Parser::new(q).unwrap();
        assert!(parser.parse().is_ok(), "Should parse: {}", q);
    }
}

#[test]
fn test_fuzz_malformed_numbers() {
    let invalid = vec![
        "SELECT * FROM facts WHERE subject = .5",
        "SELECT * FROM facts WHERE subject = 1.2.3",
    ];
    for q in invalid {
        let result = Parser::new(q).and_then(|mut p| p.parse());
        assert!(result.is_err(), "Should fail on malformed: {}", q);
    }
}

#[test]
fn test_fuzz_identifier_as_condition_value() {
    let mut parser = Parser::new("SELECT * FROM facts WHERE subject = abc").unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_fuzz_invalid_characters() {
    let mut lexer = Lexer::new("@#$%");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_fuzz_bang_without_equals() {
    let mut lexer = Lexer::new("! =");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_fuzz_comparison_operators() {
    let mut lexer = Lexer::new("< > = !=");
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() >= 4);
}

#[test]
fn test_fuzz_combined_where_clauses() {
    let queries = vec![
        "SELECT * FROM facts WHERE subject = 1 AND object = 2",
        "SELECT * FROM facts WHERE subject = 1 OR object = 2",
        "SELECT * FROM facts WHERE subject = 1 AND object = 2 AND predicate = 0",
    ];
    for q in queries {
        let mut parser = Parser::new(q).unwrap();
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse: {}\nError: {:?}",
            q,
            result.err()
        );
    }
}
