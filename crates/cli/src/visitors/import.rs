use std::collections::HashSet;
use swc_core::ecma::{ast::*, visit::Visit};

use react_intl_core::{process_import_decl, CoreState};

pub struct ImportVisitor<'a> {
    pub state: &'a CoreState,
    pub imported_names: HashSet<String>,
    pub alias_map: std::collections::HashMap<String, String>,
}

impl<'a> ImportVisitor<'a> {
    pub fn new(state: &'a CoreState) -> Self {
        Self {
            state,
            alias_map: std::collections::HashMap::new(),
            imported_names: std::collections::HashSet::new(),
        }
    }
}

impl<'a> Visit for ImportVisitor<'a> {
    fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
        let (imported_names, alias_map) = process_import_decl(import_decl, &self.state);

        // Extend instead of replace to accumulate imports from all import declarations
        self.imported_names.extend(imported_names);
        self.alias_map.extend(alias_map);
    }
}
