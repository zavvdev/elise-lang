pub mod messages;

use crate::{parser::models::expression::Expr, to_str};

pub fn analyze_fn_call_semantics<'a>(
    expr: &'a Expr,
    fn_name: &'a str,
    expected_args_len: usize,
) -> &'a Expr {
    if expected_args_len != expr.children.len() {
        panic!(
            "{}",
            messages::invalid_args_amount(
                to_str!(fn_name),
                to_str!(expected_args_len),
                to_str!(expr.children.len())
            )
        )
    }

    expr
}
