use crate::{
    Token, TokenKind, KEYWORD_ALIAS, KEYWORD_FN, KEYWORD_IMPL, KEYWORD_INTERFACE, KEYWORD_MODULE,
    KEYWORD_PROTOTYPE, KEYWORD_PUB, KEYWORD_STRUCT, KEYWORD_USE,
};

pub fn before_module_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Id { symbol } if symbol == *KEYWORD_PUB => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_USE => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_ALIAS => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_MODULE => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_FN => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_STRUCT => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_INTERFACE => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_IMPL => false,
        _ => true,
    }
}

pub fn before_use_path_item_group_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseBrace => false,
        _ => true,
    }
}

pub fn before_extern_block_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Id { symbol } if symbol == *KEYWORD_PUB => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_PROTOTYPE => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_FN => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_STRUCT => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_IMPL => false,
        _ => true,
    }
}

pub fn before_prototype_params_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseParen => false,
        _ => true,
    }
}

pub fn before_fn_params_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseParen => false,
        _ => true,
    }
}

pub fn before_struct_fields_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseBrace => false,
        _ => true,
    }
}

pub fn before_interface_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Id { symbol } if symbol == *KEYWORD_PUB => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_FN => false,
        _ => true,
    }
}

pub fn before_interface_item_fn_decl_params_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseParen => false,
        _ => true,
    }
}

pub fn before_impl_block_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Id { symbol } if symbol == *KEYWORD_PUB => false,
        TokenKind::Id { symbol } if symbol == *KEYWORD_FN => false,
        _ => true,
    }
}

pub fn before_generic_param_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::Gt => false,
        _ => true,
    }
}

pub fn before_generic_where_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::OpenBrace => false,
        _ => true,
    }
}

pub fn before_generic_where_item_condition_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Add => false,
        _ => true,
    }
}

pub fn before_generic_arg_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::Gt => false,
        _ => true,
    }
}

pub fn before_stmt(token: &Token) -> bool {
    match token.kind {
        TokenKind::CloseBrace => false,
        TokenKind::OpenBrace => false,
        TokenKind::Id { .. } => false,
        TokenKind::Literal(_) => false,
        TokenKind::OpenParen => false,
        _ => true,
    }
}

pub fn before_expr(token: &Token) -> bool {
    match token.kind {
        TokenKind::Semicolon => false,
        _ => true,
    }
}

pub fn before_expr_call_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseParen => false,
        _ => true,
    }
}

pub fn before_expr_struct_literal_field_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseBrace => false,
        _ => true,
    }
}

pub fn before_ty_fn_pointer_param_item(token: &Token) -> bool {
    match token.kind {
        TokenKind::Comma => false,
        TokenKind::CloseBrace => false,
        _ => true,
    }
}
