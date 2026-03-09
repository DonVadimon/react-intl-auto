use react_intl_core::ast::jsx::analyze_jsx_element;
use react_intl_core::types::{CoreState, TransformedMessageData, REACT_COMPONENTS};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::visitors::import::ImportVisitor;

pub struct JSXVisitor<'a> {
    pub state: &'a CoreState,
    pub imported_names: &'a std::collections::HashSet<String>,
    pub alias_map: &'a std::collections::HashMap<String, String>,
    pub messages: Vec<TransformedMessageData>,
}

impl<'a> Visit for JSXVisitor<'a> {
    fn visit_jsx_element(&mut self, element: &JSXElement) {
        element.visit_children_with(self);

        if let JSXElementName::Ident(name) = &element.opening.name {
            // Check if this is a React component, either directly or through alias
            let name_str = name.sym.as_str().to_string();
            let component_name = self.alias_map.get(&name_str).unwrap_or(&name_str);

            // Check if this is a React Intl component and it was imported
            if REACT_COMPONENTS.contains(&component_name.as_str())
                && self.imported_names.contains(&name_str)
            {
                self.process_jsx_element(element);
            }
        }
    }
}

impl<'a> JSXVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            imported_names: &import_visitor.imported_names,
            alias_map: &import_visitor.alias_map,
            messages: Vec::new(),
        }
    }

    fn process_jsx_element(&mut self, element: &JSXElement) {
        // Analyze the JSX element using shared core function
        if let Some((transformed, _needs_insertion)) = analyze_jsx_element(element, &self.state) {
            self.messages.push(transformed);
        }
    }
}
