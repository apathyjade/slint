use wasm_bindgen::prelude::*;
use shared::create_ui;
use slint::ComponentHandle;

// 当panic发生时，将错误信息输出到浏览器控制台
#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;

// 初始化panic hook
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue>  {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let ui = create_ui();
    ui.run().unwrap();
    Ok(())
}
