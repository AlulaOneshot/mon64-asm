use logos::Lexer;

use crate::{nodes::{Document, DocumentItem, Opcode}, token::Token};

pub struct Parser {

}

impl Parser {
    pub fn new() -> Parser {
        Parser {

        }
    }
    
    pub fn parse(&mut self, mut tokens: Lexer<'_, Token>) -> Option<Document> {
        let mut document: Document = Document { items: Vec::new() };

        while let Some(token) = tokens.next() {
            match token {
                Ok(Token::Directive(name)) => {
                    match name.as_str() {
                        "section" => {
                            let next = tokens.next();
                            if let Some(Ok(Token::Label(name))) = next {
                                document.items.push(DocumentItem::SectionDirective(name));
                            }
                            else {
                                eprintln!("Expected label after section directive");
                                return None;
                            }
                        },
                        "byte" => {
                            let next = tokens.next();
                            if let Some(Ok(Token::Immediate(value))) = next {
                                document.items.push(DocumentItem::ByteDirective(self.parse_immediate(value).expect("Failed to parse immediate value")));
                            }
                            else {
                                eprintln!("Expected immediate value after byte directive");
                                return None;
                            }
                        },
                        "word" => {
                            let next = tokens.next();
                            if let Some(Ok(Token::Immediate(value))) = next {
                                document.items.push(DocumentItem::WordDirective(self.parse_immediate(value).expect("Failed to parse immediate value")));
                            }
                            else {
                                eprintln!("Expected immediate value after word directive");
                                return None;
                            }
                        },
                        "dword" => {
                            let next = tokens.next();
                            if let Some(Ok(Token::Immediate(value))) = next {
                                document.items.push(DocumentItem::DWordDirective(self.parse_immediate(value).expect("Failed to parse immediate value")));
                            }
                            else {
                                eprintln!("Expected immediate value after dword directive");
                                return None;
                            }
                        },
                        "qword" => {
                            let next = tokens.next();
                            if let Some(Ok(Token::Immediate(value))) = next {
                                document.items.push(DocumentItem::QWordDirective(self.parse_immediate(value).expect("Failed to parse immediate value")));
                            }
                            else {
                                eprintln!("Expected immediate value after qword directive");
                                return None;
                            }
                        },
                        _ => {
                            eprintln!("Unexpected directive: {}", name);
                            return None;
                        }
                    }
                },
                Ok(Token::Immediate(value)) => {
                    eprintln!("Unexpected immediate value: {}", value);
                    return None;
                }
                Ok(Token::Label(name)) => {
                    document.items.push(DocumentItem::Label(name));
                    tokens.next(); // Skip :
                }
                Ok(Token::Register(name)) => {
                    eprintln!("Unexpected register: {}", name);
                    return None;
                }
                Ok(Token::Opcode(name)) => {
                    match name.as_str() {
                        "wfi" => {
                            document.items.push(DocumentItem::Instruction(Opcode::Wfi, Vec::new()));
                        },
                        _ => {

                        }
                    }
                }
                Ok(Token::Colon) => {
                    eprintln!("Unexpected colon");
                    return None;
                }
                Err(_) => {
                    eprintln!("Error parsing token");
                    return None;
                }
            }
        }

        return Some(document);
    }

    fn parse_immediate(&self, value: String) -> Option<u64> {
        if value.starts_with("0b") {
            u64::from_str_radix(&value[2..], 2).ok()
        }
        else if value.starts_with("0x") {
            u64::from_str_radix(&value[2..], 16).ok()
        }
        else {
            value.parse().ok()
        }
    }
}