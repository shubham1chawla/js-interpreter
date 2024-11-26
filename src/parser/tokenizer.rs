#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number,
    String,
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
     * Whether the tokenizer reached EOF.
     */
    fn is_eof(&self) -> bool {
        self.cursor == self.content_string.len()
    }

    /**
     * Obtains next token.
     */
    pub fn get_next_token(&mut self) -> Option<Token> {
        if !self.has_tokens() {
            return Option::None;
        }

        // Numbers:
        if self.content_string.chars().nth(self.cursor)?.is_numeric() {
            let mut number = String::new();
            for c in self.content_string[self.cursor..].chars() {
                if !c.is_numeric() {
                    break
                }
                number.push(c);
                self.cursor += 1;
            }
            return Option::Some(Token {
                token_type: TokenType::Number,
                value: number,
            })
        }

        // Strings:
        if self.content_string.chars().nth(self.cursor)? == '"' {
            let mut string = String::new();
            for c in self.content_string[self.cursor..].chars() {
                string.push(c);
                self.cursor += 1;
                if self.is_eof() || self.content_string.chars().nth(self.cursor).unwrap() == '"' {
                    break;
                }
            }
            if !self.is_eof() && self.content_string.chars().nth(self.cursor).unwrap() == '"' {
                string.push(self.content_string.chars().nth(self.cursor).unwrap());
                self.cursor += 1;
            }
            return Option::Some(Token {
                token_type: TokenType::String,
                value: string,
            })
        }

        return Option::None;
    }
}