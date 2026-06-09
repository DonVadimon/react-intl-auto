use swc_core::ecma::ast::{Expr, Lit, PropName};

#[derive(Debug, Clone)]
pub struct FieldExtractionError {
    pub field_name: String,
    pub message: String,
}

impl std::fmt::Display for FieldExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FieldExtractionError {}

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

pub fn validate_string_field(
    expr: &Expr,
    field_name: &str,
) -> Result<Option<String>, FieldExtractionError> {
    match extract_expr_string(expr) {
        Some(value) => Ok(Some(value)),
        None => {
            let expr_type = match expr {
                Expr::Ident(ident) => format!("variable '{}'", ident.sym),
                Expr::Member(_) => "member expression".to_string(),
                Expr::Call(_) => "function call".to_string(),
                Expr::Bin(_) => "binary expression".to_string(),
                Expr::Tpl(tpl) if !tpl.exprs.is_empty() => {
                    "template literal with expressions".to_string()
                }
                _ => "non-string expression".to_string(),
            };

            Err(FieldExtractionError {
                field_name: field_name.to_string(),
                message: format!(
                    "Field '{}' must be a string literal, but got {}",
                    field_name, expr_type
                ),
            })
        }
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
