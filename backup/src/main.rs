// #![no_std]
slint::include_modules!();

fn main() {
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    app.global::<AppAction>().on_change_text(move |str| {
        let app: MainWindow = weak.unwrap();
        app.set_value(str + "set_value".into())
    });

    // （可选）添加平台特定逻辑（如移动端权限请求）
    #[cfg(target_os = "android")]
    {
        // Android 平台初始化代码（如请求权限）
        println!("Running on Android");
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // Web 平台初始化代码
        println!("Running on Web");
    }
    app.run().unwrap();
}