use shared::create_ui;
use slint::ComponentHandle;
use wasm_bindgen::prelude::*;

// 初始化panic hook
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let ui = create_ui();
    ui.run().unwrap();
    Ok(())
}
