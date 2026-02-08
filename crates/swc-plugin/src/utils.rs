//! Utility functions for the SWC plugin
//!
//! Most utility functions are now provided by react-intl-core.
//! This module contains only SWC-specific utilities.

use swc_core::ecma::ast::*;

/// Creates an object property for use in AST transformations
///
/// This is a SWC-specific utility that remains in the plugin crate
/// because it deals with SWC AST types.
pub fn object_property(key: &str, value: Expr) -> Prop {
    Prop::KeyValue(KeyValueProp {
        key: PropName::Str(Str {
            span: swc_core::common::DUMMY_SP,
            value: key.into(),
            raw: None,
        }),
        value: Box::new(value),
    })
}
