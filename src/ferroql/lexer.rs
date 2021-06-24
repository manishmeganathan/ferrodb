use crate::ferroql::token::{self, Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    pub position: usize,
    pub cursor: usize,
    pub ch: char
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

impl Lexer {
    // A constructor function for the Lexer struct
    pub fn new(input: String) -> Self {
        // Prepare the input (convert to upper vector chars)
        let source = input.to_uppercase().chars().collect();
        // Create a lexer object
        let mut l = Self {
            source,
            position: 0,
            cursor: 0,
            ch: '0'
        };

        // Initialize the lexer by reading the first character
        // This will set the cursor, position and ch fields of the lexer
        l.read_char();
        // Return the created lexer
        return l;
    }  

    // A method of Lexer that lexes the source input
    pub fn lex(&mut self) -> Vec<Token> {
        // Declare a vector of Tokens
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        // Declare a accumulation Token variable
        let mut newtoken: Token;

        loop {
            // Lex the current token
            newtoken = self.next_token();
            // Check if the token is an EOF token
            if newtoken.tokentype == TokenType::EOF {
                // Push the token into the vector
                tokens.push(newtoken);
                // Break the loop
                break;
            }

            // Push the token into the vector
            tokens.push(newtoken);
        }

        // Return the vector of Tokens
        return tokens;
    }
    
    // A method of Lexer that lexes the current char 
    // into a Token and advances the lex cursor
    fn next_token(&mut self) -> Token {
        // Declare a Token
        let token: Token;
        // Consume any white space chars
        self.eat_whitespaces();

        // Check the value of the current char
        match self.ch {
            // EOF character
            '0' => {
                // Generate an EOF Token
                token = Token::new(TokenType::EOF, "".to_string());
            },
            // ALL character
            '*' => {
                // Generate an ALL Token
                token = Token::new(TokenType::ALL, "ALL".to_string());
            },
            
            // Not a single character 
            _ => {
                // Check if char is a letter
                if is_letter(self.ch) {
                    // Read full identifier
                    let ident = self.read_identifier();
                    // Convert vector char into a string
                    let identstr = ident.clone().into_iter().collect();

                    // Check if the identifer is a keyword
                    match token::generate_keyword_token(&ident) {
                        // Keyword
                        Ok(keyword_token) => {
                            // Generate the appropriate Keyword Token
                            token = Token::new(keyword_token, identstr);
                        },

                        // Identifier 
                        Err(_err) => {
                            // Generate an Identifier Token
                            token = Token::new(TokenType::IDENT, identstr);
                        }
                    }

                // Not a letter char
                } else {
                    // Generate an Illegal Token
                    token = Token::new(TokenType::ILLEGAL, self.ch.to_string());
                }
            }
        }

        // Advance the lex parser
        self.read_char();
        // Return the parsed token
        return token;
    }

    // A method of Lexer that reads the next character
    // into the lexer and advances the lex cursor
    fn read_char(&mut self) {
        // Check if the cursor has crossed the eof
        if self.cursor >= self.source.len() {
            // Set the current character to an EOF
            self.ch = '0';
        } else {
            // Set the current character after reading it
            self.ch = self.source[self.cursor];
        }

        // Move the lex position to lex cursor position
        self.position = self.cursor;
        // Move the lex cursor to the next position
        self.cursor = self.cursor + 1;
    }   

    // A method of Lexer that reads the next character
    // and returns it without changing the lexer state.
    fn peek_char(&self) -> char {
        // Check if the cursor has crossed the eof
        if self.cursor >= self.source.len() {
            // Return an EOF character
            return '0';
        } else {
            // Return the next character
            return self.source[self.cursor];
        }
    }

    // A method of Lexer that reads and consumes all white 
    // spaces until the next non whitespace character
    fn eat_whitespaces(&mut self) {
        // Iterate until character is a non white space character
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            // Read the character and consume it
            self.read_char();
        }
    }

    // A method of Lexer that reads a full identifier
    fn read_identifier(&mut self) -> Vec<char> {
        // Collect the start position of the identifier
        let start = self.position;

        // Iterate until the char is not a letter
        loop {
            if is_letter(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        // Create a slice of the source that contains the identifier
        let ident = &self.source[start..self.position];
        // Convert the char slice into vector and return it
        return ident.to_vec();
    }
}