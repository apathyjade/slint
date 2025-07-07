use wasm_bindgen::prelude::*;
use js_sys::{Array, Date};

// 当panic发生时，将错误信息输出到浏览器控制台
#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// 初始化panic hook
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body");
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));
    body.append_child(&val)?;
    Ok(())
}

// 导出一个简单的函数
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 导出一个处理字符串的函数
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 导出一个处理数组的函数
#[wasm_bindgen]
pub fn process_array(data: &[u8]) -> Vec<u8> {
    // 简单地将每个元素加倍
    data.iter().map(|&x| x * 2).collect()
}

// 导出一个处理字符串的函数
#[wasm_bindgen]
pub fn cus_log(msg: &str) {
    log(msg);
    log_u32(189);
    log_many(msg, "ok");
}