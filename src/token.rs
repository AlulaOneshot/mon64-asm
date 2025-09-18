use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("smemb|smemw|smemd|smemq|lmemb|lmemw|lmemd|lmemq|lli|lui|pushb|pushw|pushd|pushq|popb|popw|popd|popq|pushab|pushaw|pushad|pushaq|popab|popaw|popad|popaq|cbw|cbws|cwd|cwds|cdq|cdqs|movb|movw|movd|movq")]
    Opcode
}