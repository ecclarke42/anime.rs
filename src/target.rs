use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Element, HtmlCollection, HtmlElement, Node, NodeList};

// fn select(selectors: &str) -> web_sys::NodeList {
//     crate::util::document().query_selector_all(s)
// }

#[derive(Debug, PartialEq)]
pub enum Target {
    Selector(&'static str),
    // Element(Element),
    // Collection(HtmlCollection),

    // Node(Node),
    // NodeList(NodeList),

    // TODO: JSValue object?
}

impl From<&'static str> for Target {
    fn from(s: &'static str) -> Self {
        Target::Selector(s)
    }
}

impl Target {
    fn list(&self) -> Result<Vec<HtmlElement>, TargetError> {
        match self {
            Target::Selector(s) => {
                let node_list = crate::util::document()
                    .query_selector_all(s)
                    .expect("Failed to query selector"); // TODO?

                let len = node_list.length();
                let mut els = Vec::with_capacity(len as usize);
                for i in 0..len {
                    match node_list.get(i).map(JsCast::dyn_into::<HtmlElement>) {
                        Some(Ok(el)) => els.push(el),
                        Some(Err(_)) => return Err(TargetError::Convert),
                        None => return Err(TargetError::Range),
                    }
                }
                Ok(els)
            }
            // Target::Element(e) => {
            //     if let Some(el) = e.dyn_ref::<HtmlElement>() {
            //         Ok(vec![el.to_owned()])
            //     } else {
            //         Err(TargetError::Convert)
            //     }
            // }

            // // TODO
            // Target::Node(node) => Ok(vec![node.clone()],
            // Target::NodeList(list) => todo!(),
            // Target::Collection(collection) => todo!(),
        }
    }

    // pub fn get_css()

    pub fn update_css(&self, property: &str, value: &str) -> Result<(), TargetError> {
        for el in self.list()? {
            el.style()
                .set_property(property, value)
                .map(|_| TargetError::Css);
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TargetError {
    #[error("failed to convert node to HtmlElement")]
    Convert,

    #[error("Node index out of range")]
    Range,

    #[error("Failed to set CSS Value")]
    Css,
}
