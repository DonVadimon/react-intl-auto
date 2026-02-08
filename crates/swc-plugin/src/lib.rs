mod types;
mod utils;
mod visitors;

use swc_core::ecma::{
    ast::Program,
    visit::{VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use std::path::PathBuf;

use types::{PluginOptions, PluginState};
use visitors::{CallExpressionVisitor, JSXVisitor, ImportVisitor};
use std::collections::HashSet;

pub struct TransformVisitor {
    state: PluginState,
}

impl TransformVisitor {
    pub fn new(state: PluginState) -> Self {
        Self { state }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_program(&mut self, program: &mut Program) {
        // First pass: collect imported names and aliases
        let mut import_visitor = ImportVisitor {
            imported_names: HashSet::new(),
            module_source_name: self.state.opts.module_source_name.clone(),
            alias_map: std::collections::HashMap::new(),
        };
        program.visit_mut_with(&mut import_visitor);
        
        // Second pass: transform with knowledge of imports and aliases
        let mut jsx_visitor = JSXVisitor {
            state: self.state.clone(),
            imported_names: import_visitor.imported_names.clone(),
            alias_map: import_visitor.alias_map.clone(),
        };
        let mut call_visitor = CallExpressionVisitor {
            state: self.state.clone(),
            imported_names: import_visitor.imported_names,
            variable_declarations: std::collections::HashMap::new(),
        };
        
        program.visit_mut_with(&mut jsx_visitor);
        program.visit_mut_with(&mut call_visitor);
    }
}

/// Main plugin function that processes the AST
#[plugin_transform]
pub fn process_transform(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    // Parse plugin options from metadata
    let raw_config = metadata
        .get_transform_plugin_config()
        .unwrap_or_else(|| "{}".to_string());
    let opts: PluginOptions = serde_json::from_str(&raw_config).unwrap_or_default();

    // Get filename from metadata
    let filename = metadata
        .get_context(&swc_core::plugin::metadata::TransformPluginMetadataContextKind::Filename)
        .unwrap_or_else(|| "unknown.js".to_string());
    
    let state = PluginState::new(PathBuf::from(filename), opts);
    let mut visitor = TransformVisitor::new(state);

    program.visit_mut_with(&mut visitor);
    program
}

#[cfg(test)]
mod project_root_test;