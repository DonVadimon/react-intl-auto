use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use react_intl_core::{analyze_jsx_element, REACT_COMPONENTS};

use react_intl_core::CoreState;

use crate::visitors::import::ImportVisitor;

pub struct JSXVisitor<'a> {
    pub state: &'a CoreState,
    pub imported_names: &'a std::collections::HashSet<String>,
    pub alias_map: &'a std::collections::HashMap<String, String>,
}

impl<'a> VisitMut for JSXVisitor<'a> {
    fn visit_mut_jsx_element(&mut self, element: &mut JSXElement) {
        element.visit_mut_children_with(self);

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
        }
    }

    fn process_jsx_element(&mut self, element: &mut JSXElement) {
        // Analyze the JSX element using shared core function
        if let Some((transformed, needs_insertion)) = analyze_jsx_element(element, &self.state) {
            if needs_insertion {
                // ID needs to be inserted - find defaultMessage index and insert ID
                if let Some(default_message_idx) =
                    self.find_attribute_index(&element.opening, "defaultMessage")
                {
                    self.insert_id_attribute(element, default_message_idx, &transformed.id);
                }
            }
            // If needs_insertion is false, ID already exists - do nothing
        }
        // If analyze_jsx_element returns None, this is not a translatable message - do nothing
    }

    fn find_attribute_index(&self, opening: &JSXOpeningElement, name: &str) -> Option<usize> {
        opening.attrs.iter().position(|attr| {
            if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
                if let JSXAttrName::Ident(ident) = &jsx_attr.name {
                    return ident.sym.as_ref() == name;
                }
            }
            false
        })
    }

    fn insert_id_attribute(&self, element: &mut JSXElement, default_message_idx: usize, id: &str) {
        let id_attr = JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: swc_core::common::DUMMY_SP,
            name: JSXAttrName::Ident(IdentName::new("id".into(), swc_core::common::DUMMY_SP)),
            value: Some(JSXAttrValue::Str(Str {
                span: swc_core::common::DUMMY_SP,
                value: id.into(),
                raw: None,
            })),
        });

        element.opening.attrs.insert(default_message_idx, id_attr);
    }
}
