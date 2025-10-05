
use core::panic;
use std::fmt::{self, write};
use crate::lexer::Token;

pub enum NodeType {
    Program,
    Function,
    Identifier(String),
    Return,
    Expression,
    Constant(i32),
}

impl NodeType {
    pub fn check(&self, tokens: &[Token]) {
        match self {
            NodeType::Program => (),
            NodeType::Function => {
                if tokens.len() < 6 {
                    panic!("Function expects at least 6 tokens");
                }
                if &tokens[0] != &Token::Keyword("".to_string()) {
                    panic!("Function token 0 must be a Keyword");
                }
                if &tokens[1] != &Token::Identifier("".to_string()) {
                    panic!("Function token 1 must be an Identifier");
                }
                if &tokens[2] != &Token::OpenPar {
                    panic!("Function token 2 must be a OpenPar");
                }
                // arguments #####
                // -
                // ###############
                if &tokens[3] != &Token::ClosePar {
                    panic!("Function token 3 must be a ClosePar");
                }
                if &tokens[4] != &Token::OpenBrace {
                    panic!("Function token 4 must be a OpenBrace");
                }

                // statement

                let mut found_close_brace = false;
                for i in 5..tokens.len() {
                    match &tokens[i] {
                        Token::CloseBrace => {
                            found_close_brace = true;
                            break;
                        },
                        _ => ()
                    }
                }
                if !found_close_brace {
                    panic!("Function expects a CloseBrace");
                }
            },
            NodeType::Identifier(_) => (),
            NodeType::Return => {
                if tokens.len() < 3 {
                    panic!("Return expects at least 3 tokens");
                }
                match &tokens[0] {
                    Token::Keyword(name) => {
                        if name != "return" {
                            panic!("Return token 0 must be a 'return' Keyword");
                        }
                    },
                    _ => {panic!("Return token 0 must be a 'return' Keyword");}
                }

                // Expression

                let mut found_semicolon = false;
                for i in 1..tokens.len() {
                    match &tokens[i] {
                        Token::Semicolon => {
                            found_semicolon = true;
                            break;
                        },
                        _ => ()
                    }
                }
                if !found_semicolon {
                    panic!("Return expects a Semicolon");
                }
            },
            NodeType::Expression => (),
            NodeType::Constant(_) => ()
        }
    }
}

pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Node {
            node_type: node_type,
            children: Vec::<Node>::new(),
        }
    }

    pub fn parse(&mut self, tokens: &[Token], cursor: usize) -> usize{
        println!("{cursor}");

        NodeType::check(&self.node_type, &tokens[cursor..]);    
        println!("ok");


        match &self.node_type {
            NodeType::Program => {
                let mut func_node = Node::new(NodeType::Function);
                let cursor = func_node.parse(&tokens, cursor);
                self.children.push(func_node);
                return cursor;
            },
            NodeType::Function => {
                let identifier = &tokens[cursor+1];
                let mut identifier_node = match identifier {
                    Token::Identifier(name) => Node::new(NodeType::Identifier(name.clone())),
                    _ => panic!("Expected Identifier")
                };
                
                let mut cursor = identifier_node.parse(&tokens, cursor);

                let mut statement_node = Node::new(NodeType::Return);
                cursor = statement_node.parse(&tokens, cursor+5);

                self.children.push(identifier_node);
                self.children.push(statement_node);

                return cursor;
            },
            NodeType::Return => {
                let mut expression_node = Node::new(NodeType::Expression);
                let cursor = expression_node.parse(&tokens, cursor+1);

                self.children.push(expression_node);
                return cursor;
            }
            NodeType::Identifier(_) => {
                return 0;
            },
            NodeType::Expression => {
                let mut const_node = match &tokens[cursor] {
                    Token::Constant(n) => Node::new(NodeType::Constant(*n)),
                    _ => panic!("Expected Constant expression")
                };
                let cursor = const_node.parse(&tokens, cursor+1); // temp (useless)
                self.children.push(const_node);
                return cursor;
            },
            NodeType::Constant(_) => {
                return 0;
            },
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node_type {
            NodeType::Program => {
                write!(f, "Program(\n").unwrap();
                write!(f, "{}", self.children[0]).unwrap();
                write!(f, ")").unwrap();
            },
            NodeType::Function => {
                write!(f, "Function(\n").unwrap();
                write!(f, "name='{}'\n", self.children[0]).unwrap();
                write!(f, "body={}\n", self.children[1]).unwrap();
                write!(f, ")").unwrap();
            },
            NodeType::Identifier(name) => {
                write!(f, "{name}").unwrap();
            }
            NodeType::Return => {
                write!(f, "Return(\n").unwrap();
                write!(f, "{}\n", self.children[0]).unwrap();
                write!(f, ")").unwrap();
            },
            NodeType::Expression => {
                write!(f, "Expression(\n").unwrap();
                write!(f, "value={}", self.children[0]).unwrap();
                write!(f, ")").unwrap();
            },
            NodeType::Constant(n) => {
                write!(f, "{n}\n").unwrap();
            }
        }
        write!(f,"")
    }
}