use swc_core::ecma::ast::{Expr, Lit, PropName};

/// Tries to extract a string value from an expression
///
/// Supports:
/// * string literals
/// ```js
/// 'hello' // "hello"
/// ```
/// * template strings
/// ```js
/// `hello world` // ok
/// `hello ${world}` // no support
/// ```
pub fn extract_expr_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(Lit::Str(str_lit)) => Some(str_lit.value.to_string_lossy().to_string()),
        Expr::Tpl(tpl) if tpl.exprs.is_empty() && tpl.quasis.len() == 1 => {
            // Template literal with no expressions: `text`
            // TODO: use evaluator maybe https://rustdoc.swc.rs/swc_ecma_minifier/eval/struct.Evaluator.html
            let raw = &tpl.quasis[0].raw;
            if raw.is_empty() {
                None
            } else {
                Some(raw.to_string())
            }
        }
        _ => None,
    }
}

/// Extracts property name from PropName
pub fn extract_prop_name(key: &PropName) -> Option<String> {
    match key {
        PropName::Ident(ident) => Some(ident.sym.to_string()),
        PropName::Str(str_lit) => Some(str_lit.value.to_string_lossy().to_string()),
        PropName::Num(num_lit) => Some(num_lit.value.to_string()),
        _ => None,
    }
}
