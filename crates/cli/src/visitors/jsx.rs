use react_intl_core::ast::jsx::{analyze_jsx_element, is_react_intl_component};
use react_intl_core::types::{CoreState, TransformedMessageData};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::visitors::import::ImportVisitor;

pub struct JSXVisitor<'a> {
    pub messages: Vec<TransformedMessageData>,
    state: &'a CoreState,
    import_visitor: &'a ImportVisitor<'a>,
}

impl<'a> JSXVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            import_visitor,
            messages: Vec::new(),
        }
    }

    pub fn into_messages(self) -> Vec<TransformedMessageData> {
        self.messages
    }
}

impl<'a> Visit for JSXVisitor<'a> {
    fn visit_jsx_element(&mut self, element: &JSXElement) {
        element.visit_children_with(self);

        // Check if this is a React Intl component and process it
        if let JSXElementName::Ident(name) = &element.opening.name {
            if is_react_intl_component(self.import_visitor, name) {
                if let Some((transformed, _)) = analyze_jsx_element(element, self.state) {
                    self.messages.push(transformed);
                }
            }
        }
    }
}
