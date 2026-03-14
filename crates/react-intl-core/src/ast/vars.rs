//! Variable tracking utilities for AST analysis
//!
//! Provides functionality to track variable declarations with object literals
//! and resolve them in function calls. Similar to ImportCollector/ImportVisitor pattern.

use std::collections::HashMap;
use swc_core::ecma::ast::{Expr, ObjectLit, Pat, VarDeclarator};
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::types::CoreState;

/// Trait for collecting variable declarations with object literals
///
/// This trait abstracts over different implementations that can resolve
/// variable names to their object literal values.
pub trait VarCollector {
    /// Get an object literal by variable name
    fn get_object(&self, name: &str) -> Option<&ObjectLit>;
}

/// Visitor that collects variable declarations with object literals
///
/// Similar to ImportVisitor, this visitor tracks variable declarations
/// that are initialized with object literals for later resolution.
pub struct VarVisitor<'a> {
    pub state: &'a CoreState,
    pub declarations: HashMap<String, ObjectLit>,
}

impl<'a> VarVisitor<'a> {
    pub fn new(state: &'a CoreState) -> Self {
        Self {
            state,
            declarations: HashMap::new(),
        }
    }

    /// Track a variable declaration if it has an object literal initializer
    ///
    /// This method can be used directly instead of going through the Visit trait,
    /// which is useful when working with VisitMut visitors.
    pub fn track_declarator(&mut self, declarator: &VarDeclarator) {
        if let Pat::Ident(ident) = &declarator.name {
            if let Some(init) = &declarator.init {
                if let Expr::Object(obj) = init.as_ref() {
                    self.declarations.insert(ident.sym.to_string(), obj.clone());
                }
            }
        }
    }
}

impl<'a> VarCollector for &VarVisitor<'a> {
    fn get_object(&self, name: &str) -> Option<&ObjectLit> {
        self.declarations.get(name)
    }
}

impl<'a> VarCollector for VarVisitor<'a> {
    fn get_object(&self, name: &str) -> Option<&ObjectLit> {
        self.declarations.get(name)
    }
}

impl<'a> Visit for VarVisitor<'a> {
    fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
        // Track variable declarations with object literals
        self.track_declarator(declarator);

        declarator.visit_children_with(self);
    }
}
