mod utils;

use wasm_bindgen::prelude::*;
use web_sys::Document;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("HELLO")
}

fn get_dom() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("should have a document on window")
}

#[wasm_bindgen]
pub fn append() {
    // Get a reference to the DOM / Document
    let dom = get_dom();

    // Create the element we're gonna append
    let val = dom
        .create_element("p")
        .expect("MSG error");

    val.set_text_content(Some("Hello from Rust!"));

    dom .body()
        .expect("document should have a body")
        .append_child(&val).unwrap(); // Apends to the end of body
}

#[wasm_bindgen]
pub fn modify() {
    // Get a reference to the DOM / Document
    let dom = get_dom();
    let element = dom
        .get_element_by_id("my_p")
        .expect("I could not get my P element");

    element
        .set_text_content(Some("I HAVE MODIFIED THIS P"));

}
