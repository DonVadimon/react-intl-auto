use react_intl_core::ast::jsx::{analyze_jsx_element, is_react_intl_component};
use react_intl_core::types::CoreState;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use crate::visitors::import::ImportVisitor;

pub struct JSXVisitor<'a> {
    state: &'a CoreState,
    import_visitor: &'a ImportVisitor<'a>,
}

impl<'a> JSXVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            import_visitor,
        }
    }
}

impl<'a> VisitMut for JSXVisitor<'a> {
    fn visit_mut_jsx_element(&mut self, element: &mut JSXElement) {
        element.visit_mut_children_with(self);

        // Check if this is a React Intl component that needs transformation
        if let JSXElementName::Ident(name) = &element.opening.name {
            if is_react_intl_component(self.import_visitor, name) {
                // Analyze the element to determine if ID needs to be inserted
                if let Some((transformed, needs_insertion)) =
                    analyze_jsx_element(element, self.state)
                {
                    if needs_insertion {
                        // Find the position of defaultMessage attribute
                        // and insert the ID attribute right before it
                        if let Some(default_message_idx) =
                            find_attribute_index(&element.opening, "defaultMessage")
                        {
                            insert_id_attribute(element, default_message_idx, &transformed.id);
                        }
                    }
                }
            }
        }
    }
}

/// Finds the index of a JSX attribute by name
fn find_attribute_index(opening: &JSXOpeningElement, name: &str) -> Option<usize> {
    opening.attrs.iter().position(|attr| {
        if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
            if let JSXAttrName::Ident(ident) = &jsx_attr.name {
                return ident.sym.as_ref() == name;
            }
        }
        false
    })
}

/// Inserts an ID attribute at the specified position in the JSX element
fn insert_id_attribute(element: &mut JSXElement, default_message_idx: usize, id: &str) {
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
