pub struct Lexer;

/*********************************************
 * The purpose of this module is to read and
 * tokenize the input file.
 */
impl Lexer {
    pub fn tokenize(&self, content: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer: Vec<char> = Vec::new();

        for c in content.chars() {
            if c.is_alphabetic() {
                buffer.push(c);
            } else if c.is_whitespace() {
                let str: String = buffer.iter().collect();
                println!("{}",str);

                tokens.push(map_token(str));

                buffer.clear();
            }
        }

        return tokens;
    }
}

fn map_token(str: String) -> Token {
    match str.as_str() {
        "Create" => Token::new(TokenType::InitVar),
        "return" => Token::new(TokenType::Return),
        _ => panic!("Invalid token!")
    }

}

enum TokenType {
    InitVar,
    Return,
    IntLit,
    Semi,
}

pub struct Token {
    kind: TokenType,
    val: Option<String>
}

impl Token {
    fn new(kind: TokenType) -> Token {
        Token {
            kind,
            val: None
        }
    }
    // fn new(kind: TokenType, val: Option<String>) -> Token {
    //     Token {
    //         kind,
    //         val
    //     }
    // }
}



