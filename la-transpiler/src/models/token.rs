use crate::enums::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: Option<String>,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.val.is_none() {
            return write!(f, "Token ({}, None)", self.token_type)
        } else {
            return write!(f, "Token ({}, {})", self.token_type, self.val.clone().unwrap())
        }
    }
}