use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};
use swc_core::ecma::{
    ast::{ImportDecl, ImportSpecifier, ModuleExportName},
    visit::Visit,
};

use crate::types::CoreState;

pub trait ImportCollector {
    fn get_imported_names(&self) -> &HashSet<String>;
    fn get_alias_map(&self) -> &HashMap<String, String>;
}

/// ### Common visitor for cli and plugin
/// Collects imported names and aliases map
pub struct ImportVisitor<'a> {
    pub state: &'a CoreState,
    pub imported_names: HashSet<String>,
    pub alias_map: HashMap<String, String>,
}

impl<'a> ImportCollector for &ImportVisitor<'a> {
    fn get_imported_names(&self) -> &std::collections::HashSet<String> {
        &self.imported_names
    }

    fn get_alias_map(&self) -> &std::collections::HashMap<String, String> {
        &self.alias_map
    }
}

impl<'a> ImportVisitor<'a> {
    pub fn new(state: &'a CoreState) -> Self {
        Self {
            state,
            alias_map: HashMap::new(),
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

fn check_import_from(from: Cow<'_, str>, allowed: &String) -> bool {
    from == allowed.as_str() || from.starts_with(&format!("{}/", allowed))
}

pub fn process_import_decl(
    import_decl: &ImportDecl,
    state: &CoreState,
) -> (HashSet<String>, HashMap<String, String>) {
    let mut imported_names = HashSet::<String>::new();
    let mut alias_map = HashMap::<String, String>::new();

    let module_source_name = &state.opts.module_source_name;
    let from = import_decl.src.value.to_string_lossy();

    if !check_import_from(from, module_source_name) {
        return (imported_names, alias_map);
    }

    for specifier in &import_decl.specifiers {
        match specifier {
            ImportSpecifier::Named(named_spec) => {
                if let Some(imported) = &named_spec.imported {
                    match imported {
                        ModuleExportName::Ident(ident) => {
                            let original_name = ident.sym.to_string();
                            let local_name = named_spec.local.sym.to_string();

                            // Store both the imported name and local name
                            imported_names.insert(original_name.clone());
                            imported_names.insert(local_name.clone());

                            // If there's an alias, create a mapping
                            if original_name != local_name {
                                alias_map.insert(local_name, original_name);
                            }
                        }
                        ModuleExportName::Str(str_lit) => {
                            let original_name = str_lit.value.to_string_lossy().to_string();
                            let local_name = named_spec.local.sym.to_string();

                            imported_names.insert(original_name.clone());
                            imported_names.insert(local_name.clone());

                            // If there's an alias, create a mapping
                            if original_name != local_name {
                                alias_map.insert(local_name, original_name);
                            }
                        }
                    }
                } else {
                    // Default import case
                    imported_names.insert(named_spec.local.sym.to_string());
                }
            }
            ImportSpecifier::Default(default_spec) => {
                imported_names.insert("default".to_string());
                imported_names.insert(default_spec.local.sym.to_string());
            }
            ImportSpecifier::Namespace(namespace_spec) => {
                imported_names.insert(namespace_spec.local.sym.to_string());
            }
        }
    }

    (imported_names, alias_map)
}
