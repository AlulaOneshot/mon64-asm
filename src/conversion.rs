use std::collections::HashMap;

use crate::nodes::{Document, DocumentItem, Opcode, Operand};

pub fn convert_to_bytes(document: &mut Document) -> Vec<u8> {
    let mut bytecode = vec![];

    conversion_pass1(document);

    return bytecode;
}

pub fn label_resolution(document: &mut Document) -> Document {
    let mut new_document: Document = Document { items: vec![] };

    let mut label_map: HashMap<String, u64> = HashMap::new();

    let mut current_address: u64 = 0;

    for item in &document.items {
        match item {
            DocumentItem::Label(name) => {
                label_map.insert(name.clone(), current_address);
            },
            DocumentItem::Instruction(opcode, operands) => {
                match opcode {
                    Opcode::Jmp => {
                        current_address += 24;
                    },
                    Opcode::Jmpeq => {
                        current_address += 24;
                    },
                    Opcode::Jmpge => {
                        current_address += 24;
                    },
                    Opcode::Jmpgt => {
                        current_address += 24;
                    },
                    Opcode::Jmple => {
                        current_address += 24;
                    },
                    Opcode::Jmplt => {
                        current_address += 24;
                    },
                    Opcode::Jmpn => {
                        current_address += 24;
                    },
                    Opcode::Jmpneq => {
                        current_address += 24;
                    },
                    Opcode::Jmpnz => {
                        current_address += 24;
                    },
                    Opcode::Jmpo => {
                        current_address += 24;
                    },
                    Opcode::Jmpp => {
                        current_address += 24;
                    },
                    Opcode::Jmpz => {
                        current_address += 24;
                    }
                    Opcode::Call => {
                        panic!("Call not yet supported")
                    },
                    _ => {
                        current_address += 8;
                    }
                }
            }
            DocumentItem::ByteDirective(byte) => {
                current_address += 1;
            },
            DocumentItem::WordDirective(word) => {
                current_address += 2;
            }
            DocumentItem::DWordDirective(dword) => {
                current_address += 4;
            }
            DocumentItem::QWordDirective(qword) => {
                current_address += 8;
            },
            DocumentItem::SectionDirective(section) => {

            }
        }
    }

    for item in &document.items {
        match item {
            DocumentItem::Label(name) => {
                
            },
            DocumentItem::Instruction(opcode, operands) => {
                match opcode {
                    Opcode::Jmp => {
                        
                    }
                    _ => {

                    }
                }
            },
            DocumentItem::ByteDirective(byte) => {
                new_document.items.push(DocumentItem::ByteDirective(*byte));
            },
            DocumentItem::WordDirective(word) => {
                new_document.items.push(DocumentItem::WordDirective(*word));
            },
            DocumentItem::DWordDirective(dword) => {
                new_document.items.push(DocumentItem::DWordDirective(*dword));
            },
            DocumentItem::QWordDirective(qword) => {
                new_document.items.push(DocumentItem::QWordDirective(*qword));
            },
            DocumentItem::SectionDirective(section) => {
                
            }
        }
    }

    return new_document;
}