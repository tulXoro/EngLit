use std::ops::Index;

pub struct Lexer;

/*********************************************
 * The purpose of this module is to read and
 * tokenize the input file.
 *********************************************/
impl Lexer {
    // Break the file into tokens
    pub fn tokenize(&self, content: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer: Vec<char> = Vec::new();
        // Scan the entire file
        let mut i = 0;
        while i < content.len() {
            let mut token: Token = Token::new(TokenType::Illegal);
            // Check the current character
            match content.chars().nth(i).unwrap() {
                // When character is num or letter, check the next character
                'a'..='z' => {
                    while content.chars().nth(i).unwrap().is_alphabetic() {
                        buffer.push(content.chars().nth(i).unwrap());
                        i += 1;
                    }
                    token = Lexer::map_alph_token(buffer.iter().collect());
                }
                '0'..='9' => {
                    while content.chars().nth(i).unwrap().is_numeric() {
                        buffer.push(content.chars().nth(i).unwrap());
                        i += 1;
                    }
                    token = Token::new_with_val(TokenType::IntLiteral, Some(buffer.iter().collect()));
                }
                // When character is a period, create a period token
                '.' => token = Token::new(TokenType::Period),
                ' ' => {
                    // Skip whitespace
                }
                _ => {
                    println!("{}", content.chars().nth(i).unwrap());
                    panic!("Invalid character!");
                }
            }

            if token.kind != TokenType::Illegal {
                tokens.push(token);
                println!("{}", buffer.iter().collect::<String>());
                buffer.clear();
            }

            i += 1;

        }

        return tokens;
    }

    fn map_alph_token(str: String) -> Token {
        println!("Checking token: {}", str);
        match str.as_str() {
            "create" => Token::new(TokenType::InitVar),
            "end" => Token::new(TokenType::End),
            "return" => Token::new(TokenType::Return),
            _ => panic!("Invalid token!"),
        }
    }
    pub fn tokens_to_asm(&self, tokens: Vec<Token>) -> String {
        // Create a variable that can be written to a file
        // which will be the assembly code
        let mut asm: String = "_start:\n".to_string();


        for i in 0..tokens.len() {
            let token = tokens.index(i);
            match token.kind {
                TokenType::InitVar => {
                // TODO: Create a variable
                }

                TokenType::End => {


                }

                TokenType::Return => {
                    if i + 1 < tokens.len()
                    && tokens.index(i + 1).kind == TokenType::IntLiteral {

                        asm.push_str("\tmov eax, 60\n");
                        asm.push_str(&format!("\tmov rdi, {}\n",
                                              tokens.index(i + 1).val.clone().unwrap()));
                        asm.push_str("\tsyscall\n");
                    }
                }

                TokenType::Period => asm.push_str("mov eax, 1\n"),

                TokenType::Illegal => panic!("Invalid token!"),

                _ => {

                }
            }
        }

        return asm;
    }
}


#[derive(PartialEq)]
enum TokenType {
    InitVar,    // make a variable
    IntLiteral, // literal number value
    End,        // end of program
    Return,     // return value from a function
    Period,     // end of statement (like a semicolon)
    Illegal,    // invalid token
}

pub struct Token {
    kind: TokenType,
    val: Option<String>
}

impl Token {
    fn new(kind: TokenType) -> Token {
        Token { kind, val: None }
    }
    fn new_with_val(kind: TokenType, val: Option<String>) -> Token {
        Token {
            kind,
            val
        }
    }
}
