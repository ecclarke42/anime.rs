use wasm_bindgen::{prelude::*, JsCast};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("Failed to get global window")
}

pub fn document() -> web_sys::Document {
    window().document().expect("Failed to get global document")
}

pub fn document_is_hidden() -> bool {
    window().document().map(|d| d.hidden()).unwrap_or_default()
}

pub fn get_element() {
    document();
}

// TODO: replace with gloo call
pub fn request_animation_frame<F: 'static + FnMut(i32)>(f: F) -> Result<i32, JsValue> {
    let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut(i32)>);
    window().request_animation_frame(closure.as_ref().unchecked_ref())
}
