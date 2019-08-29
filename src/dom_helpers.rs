use js_sys::Error;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, Window};

pub fn get_window() -> Result<Window, Error> {
    web_sys::window().ok_or_else(|| Error::new("Can't get the window"))
}

pub fn get_document(window: &Window) -> Result<Document, Error> {
    window
        .document()
        .ok_or_else(|| Error::new("Can't get the document"))
}

pub fn get_body(document: &Document) -> Result<HtmlElement, Error> {
    document
        .body()
        .ok_or_else(|| Error::new("Can't get the body"))
}

pub fn create_canvas(document: &Document) -> Result<HtmlCanvasElement, Error> {
    document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| {
            Error::new(&format!(
                "Can't cast created element to HtmlCanvasElement, {:?}",
                e
            ))
        })
}

pub fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, Error> {
    canvas
        .get_context("2d")?
        .ok_or_else(|| Error::new("Can't get rendering context 2d"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|e| {
            Error::new(&format!(
                "Can't cast rendering context to WebGlRenderingContext2d, {:?}",
                e
            ))
        })
}
