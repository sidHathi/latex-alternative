pub mod Ast {
    use crate::models::ast_node::AstNode;
    use crate::models::token::Token;
    use crate::enums::TokenType;
    use crate::constants::syntaxkeys::functional_tokentypes;
    use std::collections::HashSet;
    use std::rc::Rc;
    use std::cell::RefCell;

    fn functional_token(token: &Token) -> bool {
        let fn_tokenset: HashSet<TokenType> = HashSet::from(functional_tokentypes);
        fn_tokenset.contains(&token.token_type)
    }

    fn exit_token(candidate: &Token, curr_node: Rc<RefCell<AstNode>>) -> bool {
        match curr_node.borrow().token.token_type {
            TokenType::BLOCK => {
                if candidate.token_type == TokenType::BACKTAB {
                    return true;
                }
            },
            TokenType::EQBLOCK => {
                if candidate.token_type == TokenType::BACKTAB {
                    return true;
                }
            },
            TokenType::EQ => {
                if candidate.token_type == TokenType::EQEND {
                    return true;
                }
            },
            TokenType::STYLE => {
                if candidate.token_type == TokenType::CLOSEPAREN {
                    return true;
                }
            },
            TokenType::LXFUNC => {
                return true;
            },
            TokenType::ENUMITEM => return true,
            _ => ()
        };
        return false;
    }
    
    pub fn ast_construct(token_list: Vec<Token>) -> Option<Rc<RefCell<AstNode>>> {
        if token_list.len() < 1 {
            return None;
        }

        let head: Rc<RefCell<AstNode>> = Rc::new(RefCell::new(AstNode::new((*token_list.first().unwrap()).clone())));
        let mut curr_node: Rc<RefCell<AstNode>> = Rc::clone(&head);
        let mut parent_trace: Vec<Rc<RefCell<AstNode>>> = Vec::new();
        for token in &token_list[1..] {
            let child = Rc::new(RefCell::new(AstNode::new(token.clone())));
            if functional_token(token) {
                curr_node.borrow_mut().add_child(Rc::clone(&child));
                let parent_node: Rc<RefCell<AstNode>> = Rc::clone(&curr_node);
                parent_trace.push(parent_node);
                curr_node = child;
            } else if token.token_type == TokenType::PARAM {
                curr_node.borrow_mut().add_param(token.clone());
            } else if exit_token(token, Rc::clone(&curr_node)){
                if parent_trace.len() > 0 {
                    curr_node = parent_trace.pop().unwrap();
                }
            } else {
                curr_node.borrow_mut().add_child(Rc::clone(&child));
            }
        }
        return Some(head);
    }
}