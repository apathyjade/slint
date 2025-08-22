

slint::include_modules!();

// 暴露 UI 组件和初始化函数（供平台入口调用）
pub fn create_ui() -> MainWindow {
  MainWindow::new().unwrap()
}