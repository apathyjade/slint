#![no_std]
slint::include_modules!();

fn main() {
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    app.global::<AppAction>().on_change_text(move |str| {
        let app: MainWindow = weak.unwrap();
        app.set_value("set_value".into())
    });

    app.run().unwrap();
}