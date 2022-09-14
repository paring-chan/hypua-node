use wasm_bindgen::prelude::wasm_bindgen;

extern crate alloc;
extern crate hypua;

#[wasm_bindgen]
pub fn to_ipf_string(s: &str) -> String {
    let ipf = hypua::to_ipf_string(s);

    String::from(ipf)
}
