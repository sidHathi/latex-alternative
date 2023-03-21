use crate::enums::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: Option<String>,
    pub start_index: usize,
    pub end_index: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType, 
        val: Option<String>,
        start_index: usize,
        end_index: usize,
    ) -> Token {
        Token {
            token_type: token_type,
            val: val,
            start_index: start_index,
            end_index: end_index,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.val.is_none() {
            return write!(f, "Token ({}, None, {}, {})\n", self.token_type, self.start_index, self.end_index)
        } else {
            return write!(f, "Token ({}, {}, {}, {})\n", self.token_type, self.val.clone().unwrap(), self.start_index, self.end_index)
        }
    }
}