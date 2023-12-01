use exc_symbol::Symbol;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref UNKNOWN: Symbol = Symbol::from_str("unknown");
    pub static ref WHITESPACE: Symbol = Symbol::from_str("whitespace");
    pub static ref COMMENT: Symbol = Symbol::from_str("comment");
    pub static ref OPEN_PAREN: Symbol = Symbol::from_str("(");
    pub static ref CLOSE_PAREN: Symbol = Symbol::from_str(")");
    pub static ref OPEN_BRACE: Symbol = Symbol::from_str("{");
    pub static ref CLOSE_BRACE: Symbol = Symbol::from_str("}");
    pub static ref OPEN_BRACKET: Symbol = Symbol::from_str("[");
    pub static ref CLOSE_BRACKET: Symbol = Symbol::from_str("]");
    pub static ref DOT: Symbol = Symbol::from_str(".");
    pub static ref COMMA: Symbol = Symbol::from_str(",");
    pub static ref COLON: Symbol = Symbol::from_str(":");
    pub static ref SEMICOLON: Symbol = Symbol::from_str(";");
    pub static ref ARROW: Symbol = Symbol::from_str("->");
}

lazy_static! {
    pub static ref ASSIGN: Symbol = Symbol::from_str("=");
    pub static ref ASSIGN_ADD: Symbol = Symbol::from_str("+=");
    pub static ref ASSIGN_SUB: Symbol = Symbol::from_str("-=");
    pub static ref ASSIGN_MUL: Symbol = Symbol::from_str("*=");
    pub static ref ASSIGN_DIV: Symbol = Symbol::from_str("/=");
    pub static ref ASSIGN_MOD: Symbol = Symbol::from_str("%=");
    pub static ref ASSIGN_POW: Symbol = Symbol::from_str("**=");
    pub static ref ASSIGN_SHL: Symbol = Symbol::from_str("<<=");
    pub static ref ASSIGN_SHR: Symbol = Symbol::from_str(">>=");
    pub static ref ASSIGN_BIT_OR: Symbol = Symbol::from_str("|=");
    pub static ref ASSIGN_BIT_AND: Symbol = Symbol::from_str("&=");
    pub static ref ASSIGN_BIT_XOR: Symbol = Symbol::from_str("^=");
    pub static ref ASSIGN_BIT_NOT: Symbol = Symbol::from_str("~=");
    pub static ref RNG: Symbol = Symbol::from_str("..");
    pub static ref RNG_INCLUSIVE: Symbol = Symbol::from_str("..=");
    pub static ref EQ: Symbol = Symbol::from_str("==");
    pub static ref NE: Symbol = Symbol::from_str("!=");
    pub static ref LT: Symbol = Symbol::from_str("<");
    pub static ref GT: Symbol = Symbol::from_str(">");
    pub static ref LE: Symbol = Symbol::from_str("<=");
    pub static ref GE: Symbol = Symbol::from_str(">=");
    pub static ref ADD: Symbol = Symbol::from_str("+");
    pub static ref SUB: Symbol = Symbol::from_str("-");
    pub static ref MUL: Symbol = Symbol::from_str("*");
    pub static ref DIV: Symbol = Symbol::from_str("/");
    pub static ref MOD: Symbol = Symbol::from_str("%");
    pub static ref POW: Symbol = Symbol::from_str("**");
    pub static ref SHL: Symbol = Symbol::from_str("<<");
    pub static ref SHR: Symbol = Symbol::from_str(">>");
    pub static ref BIT_OR: Symbol = Symbol::from_str("|");
    pub static ref BIT_AND: Symbol = Symbol::from_str("&");
    pub static ref BIT_XOR: Symbol = Symbol::from_str("^");
    pub static ref LOG_OR: Symbol = Symbol::from_str("||");
    pub static ref LOG_AND: Symbol = Symbol::from_str("&&");
    pub static ref BIT_NOT: Symbol = Symbol::from_str("~");
    pub static ref LOG_NOT: Symbol = Symbol::from_str("!");
    pub static ref PATH_SEP: Symbol = Symbol::from_str("::");
    pub static ref ID: Symbol = Symbol::from_str("identifier");
    pub static ref LITERAL: Symbol = Symbol::from_str("literal");
}

lazy_static! {
    pub static ref KEYWORD_USE: Symbol = Symbol::from_str("use");
    pub static ref KEYWORD_SELF: Symbol = Symbol::from_str("self");
    pub static ref KEYWORD_SUPER: Symbol = Symbol::from_str("super");
    pub static ref KEYWORD_ALIAS: Symbol = Symbol::from_str("alias");
    pub static ref KEYWORD_MODULE: Symbol = Symbol::from_str("module");
    pub static ref KEYWORD_EXTERN: Symbol = Symbol::from_str("extern");
    pub static ref KEYWORD_PROTOTYPE: Symbol = Symbol::from_str("prototype");
    pub static ref KEYWORD_FN: Symbol = Symbol::from_str("fn");
    pub static ref KEYWORD_STRUCT: Symbol = Symbol::from_str("struct");
    pub static ref KEYWORD_INTERFACE: Symbol = Symbol::from_str("interface");
    pub static ref KEYWORD_IMPL: Symbol = Symbol::from_str("impl");
    pub static ref KEYWORD_PUB: Symbol = Symbol::from_str("pub");
    pub static ref KEYWORD_WHERE: Symbol = Symbol::from_str("where");
    pub static ref KEYWORD_LET: Symbol = Symbol::from_str("let");
    pub static ref KEYWORD_IF: Symbol = Symbol::from_str("if");
    pub static ref KEYWORD_ELSE: Symbol = Symbol::from_str("else");
    pub static ref KEYWORD_LOOP: Symbol = Symbol::from_str("loop");
    pub static ref KEYWORD_WHILE: Symbol = Symbol::from_str("while");
    pub static ref KEYWORD_BREAK: Symbol = Symbol::from_str("break");
    pub static ref KEYWORD_CONTINUE: Symbol = Symbol::from_str("continue");
    pub static ref KEYWORD_RETURN: Symbol = Symbol::from_str("return");
    pub static ref KEYWORD_AS: Symbol = Symbol::from_str("as");
}

lazy_static! {
    pub static ref TYPENAME_BOOL: Symbol = Symbol::from_str("bool");
    pub static ref TYPENAME_INT: Symbol = Symbol::from_str("int");
    pub static ref TYPENAME_FLOAT: Symbol = Symbol::from_str("float");
    pub static ref TYPENAME_STRING: Symbol = Symbol::from_str("string");
    pub static ref TYPENAME_PTR: Symbol = Symbol::from_str("ptr");
    pub static ref TYPENAME_REF: Symbol = Symbol::from_str("ref");
}
