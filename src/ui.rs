
// 默认包含slint生成的模块

slint::include_modules!();    
use slint::ComponentHandle;

#[allow(dead_code)]
pub fn create_ui() -> MainWindow {
    let app = MainWindow::new().unwrap();
    app.run().unwrap();
    app
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_ui() {}
}