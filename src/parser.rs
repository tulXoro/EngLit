use crate::lexer::Token;

/*********************************************
 * The purpose of this module is to parse
 * tokens into an abstract syntax tree.
 *********************************************/

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    fn peak(&self) -> Option<Token> {
        self.tokens[self.index].clone()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens[self.index+1].clone()
    }

    fn eat(&mut self) -> Option<Token> {
        self.index += 1;
        self.tokens[self.index].clone()
    }
}

struct NodeExpr {
    val: Token::IntLiteral,
}
