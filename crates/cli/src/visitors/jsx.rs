use react_intl_core::ast::jsx::{analyze_jsx_element, is_react_intl_component};
use react_intl_core::types::{CoreState, TransformedMessageData};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::visitors::import::ImportVisitor;

pub struct JSXVisitor<'a> {
    pub messages: Vec<TransformedMessageData>,
    pub errors: Vec<String>,
    state: &'a CoreState,
    import_visitor: &'a ImportVisitor<'a>,
}

impl<'a> JSXVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            import_visitor,
            messages: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn into_result(self) -> (Vec<TransformedMessageData>, Vec<String>) {
        (self.messages, self.errors)
    }
}

impl<'a> Visit for JSXVisitor<'a> {
    fn visit_jsx_element(&mut self, element: &JSXElement) {
        element.visit_children_with(self);

        // Check if this is a React Intl component and process it
        if let JSXElementName::Ident(name) = &element.opening.name {
            if is_react_intl_component(self.import_visitor, name) {
                match analyze_jsx_element(element, self.state) {
                    Ok(Some((transformed, _))) => {
                        self.messages.push(transformed);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        self.errors
                            .push(format!("Error analyzing JSX element: {}", e));
                    }
                }
            }
        }
    }
}
