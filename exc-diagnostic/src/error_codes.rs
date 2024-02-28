pub const _PHASE_PARSE: u32 = 10000;
pub const UNEXPECTED_TOKEN: u32 = 10001;
pub const UNEXPECTED_EOF: u32 = 10002;
pub const INVALID_ASSIGNMENT_OPERATOR: u32 = 10003;
pub const INVALID_BINARY_OPERATOR: u32 = 10004;
pub const INVALID_UNARY_OPERATOR: u32 = 10005;

pub const _PHASE_RESOLUTION: u32 = 20000;
pub const UNREACHABLE_MODULE: u32 = 20001;
pub const MODULE_NOT_FOUND: u32 = 20002;
pub const SYMBOL_NOT_FOUND: u32 = 20003;
pub const DUPLICATED_MODULE: u32 = 20004;
pub const DUPLICATED_SYMBOL: u32 = 20005;
pub const MODULE_HAS_NO_SUPER: u32 = 20006;
pub const MODULE_IS_NOT_VISIBLE: u32 = 20007;
pub const SYMBOL_IS_NOT_VISIBLE: u32 = 20008;