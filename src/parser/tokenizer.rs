use std::fmt::format;

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // ----- SPECIAL -----
    EOF,

    // ----- LITERALS -----
    Number,
    String,

    // ----- SYMBOLS & DELIMITERS -----
    SemiColon,
    CurlyBracketOpen,
    CurlyBracketClose,
    CircleBracketOpen,
    CircleBracketClose,
    Comma,

    // ----- OPERATORS -----
    AdditiveOperator,
    MultiplicativeOperator,
    RelationalOperator,
    SimpleAssignmentOperator,
    ComplexAssignmentOperator,
    EqualityOperator,
    LogicalAndOperator,
    LogicalOrOperator,
    LogicalNotOperator,

    // ----- IDENTIFIERS -----
    Identifier,

    // ----- KEYWORDS -----
    LetKeyword,
    IfKeyword,
    ElseKeyword,
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
}

impl TokenType {
    /**
     * Tokenizer spec.
     */
    const SPEC: [(Option<TokenType>, &str); 28] = [
        // ----- WHITESPACES -----
        (None, r"^\s+"),

        // ----- SINGLE-LINE COMMENTS -----
        (None, r"^(//.*)"),

        // ----- MULTI-LINE COMMENTS -----
        (None, r"^(/*[\s\S]*?\*/)"),

        // ----- SYMBOLS & DELIMITERS -----
        (Some(Self::SemiColon), r"^(;)"),
        (Some(Self::CurlyBracketOpen), r"^(\{)"),
        (Some(Self::CurlyBracketClose), r"^(\})"),
        (Some(Self::CircleBracketOpen), r"^(\()"),
        (Some(Self::CircleBracketClose), r"^(\))"),
        (Some(Self::Comma), r"^(\,)"),

        // ----- KEYWORDS -----
        (Some(Self::LetKeyword), r"^(\blet\b)"),
        (Some(Self::IfKeyword), r"^(\bif\b)"),
        (Some(Self::ElseKeyword), r"^(\belse\b)"),
        (Some(Self::TrueKeyword), r"^(\btrue\b)"),
        (Some(Self::FalseKeyword), r"^(\bfalse\b)"),
        (Some(Self::NullKeyword), r"^(\bnull\b)"),

        // ----- NUMBERS -----
        (Some(Self::Number), r"^(\d+)"),

        // ----- STRINGS -----
        (Some(Self::String), r#"^(".*?")"#),
        (Some(Self::String), r#"^('.*?')"#),

        // ----- IDENTIFIERS -----
        (Some(Self::Identifier), r"^(\w+)"),

        // ----- EQUALITY OPERATORS -----
        (Some(Self::EqualityOperator), r"^([=|!]=)"),

        // ----- LOGICAL OPERATORS -----
        (Some(Self::LogicalAndOperator), r"^(&&)"),
        (Some(Self::LogicalOrOperator), r"^(\|\|)"),
        (Some(Self::LogicalNotOperator), r"^(!)"),

        // ----- ASSIGNMENT OPERATORS -----
        (Some(Self::SimpleAssignmentOperator), r"^(=)"),
        (Some(Self::ComplexAssignmentOperator), r"^([\+|\-|\*|/]=)"),

        // ----- MATH OPERATORS -----
        (Some(Self::AdditiveOperator), r"^(\+|-)"),
        (Some(Self::MultiplicativeOperator), r"^(\*|/)"),

        // ----- RELATIONAL OPERATORS -----
        (Some(Self::RelationalOperator), r"^([><]=?)"),
    ];
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

/**
 * Tokenizer class.
 * 
 * Lazily pulls a token from a stream.
 */
pub struct Tokenizer {
    content_string: String,
    cursor: usize,
}

impl Tokenizer {

    pub fn new(content_string: String) -> Self {
        Self {
            content_string,
            cursor: 0,
        }
    }

    /**
     * Whether we still have more tokens.
     */
    fn has_tokens(&self) -> bool {
        self.cursor < self.content_string.len()
    }

    /**
     * Obtains next token.
     */
    pub fn get_next_token(&mut self) -> Result<Token, SyntaxError> {
        if !self.has_tokens() {
            return Ok(Token {
                token_type: TokenType::EOF,
                value: String::new(),
            });
        }

        for (token_type, regex) in TokenType::SPEC {
            let re = Regex::new(regex).unwrap();
            if let Some(caps) = re.captures(&self.content_string[self.cursor..]) {
                let cap = &caps[0];
                self.cursor += cap.len();

                // Should skip token, e.g. whitespaces
                return match token_type {
                    None => self.get_next_token(),
                    Some(token_type) => Ok(Token {
                        token_type,
                        value: cap.to_string(),
                    })
                };
            }
        }

        Err(SyntaxError {
            message: format(format_args!("Unexpected token: {}", self.content_string.chars().nth(self.cursor).unwrap())),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntaxError {
    pub message: String,
}
