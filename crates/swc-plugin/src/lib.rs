mod visitors;

use react_intl_core::types::{CoreOptions, CoreState};
use std::path::PathBuf;
use swc_core::ecma::{
    ast::Program,
    visit::{VisitMut, VisitMutWith, VisitWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

use crate::visitors::call::CallExpressionVisitor;
use crate::visitors::import::ImportVisitor;
use crate::visitors::jsx::JSXVisitor;
use crate::visitors::vars::VarVisitor;

pub struct TransformVisitor {
    state: CoreState,
}

impl TransformVisitor {
    pub fn new(state: CoreState) -> Self {
        Self { state }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_program(&mut self, program: &mut Program) {
        // First pass: collect imported names and aliases
        let mut import_visitor = ImportVisitor::new(&self.state);
        let mut var_visitor = VarVisitor::new(&self.state);
        program.visit_with(&mut import_visitor);
        program.visit_with(&mut var_visitor);

        // Second pass: transform with knowledge of imports and aliases
        let mut jsx_visitor = JSXVisitor::new(&self.state, &import_visitor);
        let mut call_visitor =
            CallExpressionVisitor::new(&self.state, &import_visitor, &var_visitor);

        program.visit_mut_with(&mut jsx_visitor);
        program.visit_mut_with(&mut call_visitor);
    }
}

/// Main plugin function that processes the AST
#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    // Parse plugin options from metadata
    let raw_config = metadata
        .get_transform_plugin_config()
        .unwrap_or_else(|| "{}".to_string());
    let opts: CoreOptions = serde_json::from_str(&raw_config).unwrap_or_default();

    // Get filename from metadata
    let filename = metadata
        .get_context(&swc_core::plugin::metadata::TransformPluginMetadataContextKind::Filename)
        .unwrap_or_else(|| "unknown.js".to_string());

    let state = CoreState::new(PathBuf::from(filename), opts);
    let mut visitor = TransformVisitor::new(state);

    program.visit_mut_with(&mut visitor);
    program
}
