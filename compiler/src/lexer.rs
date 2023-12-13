// internal
pub use crate::utils::cleanup;

// std
use std::collections::HashMap;

struct Lexer {
    // Any known keywords that need to have spaces in between them
    spaced_words: HashMap<&'static str, Tokens::Token>,

    // Any known operators / characters that do not need spaces
    operators: HashMap<&'static str, Tokens::Token>,

    // Token types for identifiers and literals
    integer_token: Tokens::Token,
    identifier_token: Tokens::Token,
}

impl Lexer {
    // Token types for identifiers and literals
    
    pub fn new() -> Lexer {
        let spaced_words = [
            // Types
            ("i32", Tokens::Token::new(Tokens::TokenType::I32)),

            // Keywords
            ("let", Tokens::Token::new(Tokens::TokenType::LET)),
        ].into_iter().collect();

        let operators = [
            ("+", Tokens::Token::new(Tokens::TokenType::ADD)),
            ("-", Tokens::Token::new(Tokens::TokenType::SUB)),
            ("*", Tokens::Token::new(Tokens::TokenType::MUL)),
            ("/", Tokens::Token::new(Tokens::TokenType::DIV)),
            ("%", Tokens::Token::new(Tokens::TokenType::MOD)),
            ("(", Tokens::Token::new(Tokens::TokenType::LPAREN)),
            (")", Tokens::Token::new(Tokens::TokenType::RPAREN)),
        ].into_iter().collect();
        
        let this = Lexer {spaced_words: spaced_words, operators: operators, 
            integer_token: Tokens::Token::new(Tokens::TokenType::INT),
            identifier_token: Tokens::Token::new(Tokens::TokenType::IDENT),
        };

        this  
    }

    pub fn scan(&mut self, data: &mut String) {
        // Tries to match operator first
        // Does this by looping through string views until largest operator size and trying to match
        // Tries to match spaced_words next
        // Just goes until next space and compares
        // Tries to match identifies and literals last
        // Again goes until space
        

        // utils::cleanup(data);
    }
} 


mod Tokens {
    #[derive(Default)]
    pub enum TokenType {
        // Operators / characters
        ADD,
        SUB,
        MUL,
        DIV,
        MOD,
        LPAREN,
        RPAREN,

        // Types
        // Integers
        I8,
        I16,
        I32,
        I64,

        // Unsigned
        U1, // Bool
        U8,
        U16,
        U32, 
        U64,

        // Float
        F32,
        F64,

        // Literals
        INT,
        FLOAT,
        STR,

        // Identifiers (unknown names)
        IDENT,

        // Keywords
        LET,

        #[default]
        NONE,
    }

    #[derive(Default)]
    pub struct Token {
        ttype: TokenType,
        value: String,
    }

    impl Token {
        pub fn new(ttype: TokenType) -> Token {
            Token {ttype: ttype, ..Default::default()}
        }
    }

    pub struct TokenManager {
        tokens: Vec<Token>,
    }
}
