pub struct Lexer;

/*********************************************
 * The purpose of this module is to read and
 * tokenize the input file.
 *********************************************/
impl Lexer {
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
                    token = Token::new(TokenType::IntLiteral);
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
            _ => panic!("Invalid token!"),
        }
    }
    fn tokens_to_asm(&self, tokens: Vec<Token>) -> String {
        let mut asm: String = String::new();

        for token in tokens {
            match token.kind {
                TokenType::InitVar => asm.push_str("mov eax, 0\n"),
                TokenType::IntLiteral => asm.push_str("mov eax, 1\n"),
                TokenType::End => asm.push_str("mov eax, 1\n"),
                TokenType::Period => asm.push_str("mov eax, 1\n"),

                TokenType::Illegal => panic!("Invalid token!"),

                _ => panic!("Invalid token!"),
            }
        }

        return asm;
    }
}


#[derive(PartialEq)]
enum TokenType {
    InitVar,
    IntLiteral,
    End,
    Period,
    Illegal,
}

pub struct Token {
    kind: TokenType,
    val: Option<String>,
}

impl Token {
    fn new(kind: TokenType) -> Token {
        Token { kind, val: None }
    }
    // fn new(kind: TokenType, val: Option<String>) -> Token {
    //     Token {
    //         kind,
    //         val
    //     }
    // }
}
