pub mod tokenizer;
pub mod keywords;
pub mod constants;
pub mod enums;
pub mod models;

pub mod transpiler {
    use strum::IntoEnumIterator; // 0.17.1
    use crate::enums::TokenType;
    use crate::tokenizer::tokenizer::tokenize;

    pub fn lib_hello() -> &'static str {
        "hello world"
    }
    
    pub fn enumerate_token_types() -> Vec<TokenType> {
        let mut token_types: Vec<TokenType> = Vec::new();
        for token_type in TokenType::iter() {
            token_types.push(token_type);
        }
        token_types
    }

    pub fn tokenize_string(raw: String) -> String {
        return tokenize(raw).into_iter().map(|s| s.to_string()).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
