use crate::enums::TokenType;
use crate::models::token::Token;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::format;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct AstNode {
    pub token: Token,
    pub children: Vec<Rc<RefCell<AstNode>>>,
    pub params: Vec<Token>,
}

impl AstNode {
    pub fn new(token: Token) -> AstNode {
        AstNode {
            token: token,
            children: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn add_param(&mut self, param: Token) {
        self.params.push(param);
    }

    pub fn add_child(&mut self, new_child: Rc<RefCell<AstNode>>) {
        self.children.push(new_child);
    }

    pub fn to_string(&self) -> String {
        let mut node_string = format!("{}", self.token.clone());
        node_string.push_str("params: ");
        for param in self.params.clone() {
            node_string.push_str(format!("{}", param.clone()).as_str());
        }
        node_string.push_str(format!("children({}): ", self.children.len()).as_str());
        for child in self.children.clone() {
            node_string.push_str(format!("{}", child.borrow()).as_str());
        }
        return node_string;
    }

    fn primary_latex(&self) -> String {
        match self.token.token_type {
            TokenType::BLOCK => {
                let mut blockstr: String = "\\begin".to_owned();
                for param in self.params.clone() {
                    blockstr.push_str(format!("{{{}}}", param.val.clone().unwrap()).as_str());
                }
                blockstr.push_str("\n");
                return blockstr;
            },
            TokenType::EQ => "$".to_owned(),
            TokenType::EQBLOCK => "\\begin{equation}\\begin{split}".to_owned(),
            TokenType::LXFUNC => {
                let mut funcstr = format!("\\{}", self.token.val.clone().unwrap());
                for param in self.params.clone() {
                    funcstr.push_str(format!("{{{}}}", param.val.clone().unwrap()).as_str());
                }
                return funcstr;
            },
            TokenType::LXSYMBOL => format!("\\{}", self.token.val.clone().unwrap()),
            TokenType::ENUMITEM => "\\item".to_owned(), // TO-DO: integrate number selection
            TokenType::STYLE => format!("\\text{}{{", self.token.val.clone().unwrap()),
            _ => {
                if !self.token.val.is_none() {
                    return self.token.val.clone().unwrap();
                }
                return "".to_owned();
            }
        }
    }

    fn inner_latex(&self) -> String {
        if self.children.len() > 0 {
            let mut childStr: String = "".to_owned();
            for child in self.children.clone() {
                childStr.push_str(child.borrow().to_latex().as_str());
                childStr.push_str(" ");
            }
            return childStr.to_owned();
        }
        return "".to_owned();
    }

    fn exit_latex(&self) -> String {
        match self.token.token_type {
            TokenType::BLOCK => {
                let mut blockstr: String = "\\end".to_owned();
                for param in self.params.clone() {
                    blockstr.push_str(format!("{{{}}}", param.val.clone().unwrap()).as_str());
                }
                blockstr.push_str("\n");
                return blockstr;
            },
            TokenType::EQ => "$".to_owned(),
            TokenType::EQBLOCK => "\\end{split}\\end{equation}".to_owned(),
            TokenType::STYLE => "}".to_owned(),
            _ => {
                return "".to_owned();
            }
        }
    }

    pub fn to_latex(&self) -> String {
        let mut latex: String = self.primary_latex();
        latex.push_str(self.inner_latex().as_str());
        latex.push_str(self.exit_latex().as_str());
        return latex;
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}