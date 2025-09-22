use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"smemb|smemw|smemd|smemq|lmemb|lmemw|lmemd|lmemq|lli|lui|pushb|pushw|pushd|pushq|popb|popw|popd|popq|pushab|pushaw|pushad|pushaq|popab|popaw|popad|popaq|cbw|cbws|cwd|cwds|cdq|cdqs|movb|movw|movd|movq|addb|addw|addd|addq|addbs|addws|addds|addqs|subb|subw|subd|subq|subbs|subws|subds|subqs|mulb|mulw|muld|mulq|mulbs|mulws|mulds|mulqs|divb|divw|divd|divq|divbs|divws|divds|divqs|incb|incw|incd|incq|incbs|incws|incds|incqs|decb|decw|decd|decq|decbs|decws|decds|decqs|negb|negw|negd|negq|cmpb|cmpw|cmpd|cmpq|cmpbs|cmpws|cmpds|cmpqs|andb|andw|andd|andq|orb|orw|ord|orq|xorb|xorw|xord|xorq|notb|notw|notd|notq|norb|norw|nord|norq|nandb|nandw|nandd|nandq|shlb|shlw|shld|shlq|slrb|shrw|shrd|shrq|rolb|rolw|rold|rolq|rorb|rorw|rord|rorq|bitt|bits|bitc|jmp|jmpeq|jmpz|jmpneq|jmpnz|jmpgt|jmpge|jmplt|jmple|jmpo|jmpn|jmpp|call|ret|int|wfi|rst|inb|inw|ind|inq|outb|outw|outd|outq|nop|cpuid", |lex| lex.slice().to_owned())]
    Opcode(String),
    #[regex(r"%(r0|r1|r2|r3|r4|r5|r6|r7|r8|r9|r10|r11|r12|r12|r13|r14|r15|rflags|rip|rsp|rpt|rit|cr0|cr1|imm0|imm1|imm2|imm3|imm4|imm5|imm6|imm7)", |lex| lex.slice().to_owned())]
    Register(String),
    #[regex(r"\$(0x[0-9a-fA-F]+)|([0-9]+)|(0b[01]+)", |lex| lex.slice().to_owned())]
    Immediate(String),
    #[regex(r"\.?[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Label(String),
    #[regex(r"section|byte|word|dword|qword", |lex| lex.slice().to_owned())]
    Directive(String),
    #[token(":")]
    Colon,
}