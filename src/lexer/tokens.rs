use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    // Keywords
    //// Types
    Int,
    //// Control flow
    Ret,
    // Symbols
    LP,
    RP,
    LB,
    RB,
    Semi,
    Colon,
    Assign,
    // Complex Tokens
    Id,
    IntLit,
    // Special Tokens
    EOF,
    UNKNOWN,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: u32,
    pub column: u32,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{{{:?}, {:?}, ({:?}, {:?})}}",
            self.token_type, self.value, self.line, self.column
        )
    }
}

// Consts in rust are compile time constants, and are immutable. Hashmaps are
// complex data structures that require heap allocation, which cannot be done
// at compile time. So, we use OnceLock to create a mutable static variable
// that can be initialized at runtime and then used as a constant.
// (Kinda similar to a singleton pattern, but not quite)
static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();
fn init_keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords: HashMap<&'static str, TokenType> = HashMap::new();
    keywords.insert("int", TokenType::Int);
    keywords.insert("return", TokenType::Ret);
    keywords
}
pub fn get_keywords() -> &'static HashMap<&'static str, TokenType> {
    KEYWORDS.get_or_init(init_keywords)
}

static SYMBOLS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();
fn init_symbols() -> HashMap<&'static str, TokenType> {
    let mut symbols: HashMap<&'static str, TokenType> = HashMap::new();
    symbols.insert("(", TokenType::LP);
    symbols.insert(")", TokenType::RP);
    symbols.insert("{", TokenType::LB);
    symbols.insert("}", TokenType::RB);
    symbols.insert(";", TokenType::Semi);
    symbols.insert(":", TokenType::Colon);
    symbols.insert("=", TokenType::Assign);
    symbols
}
pub fn get_symbols() -> &'static HashMap<&'static str, TokenType> {
    SYMBOLS.get_or_init(init_symbols)
}
