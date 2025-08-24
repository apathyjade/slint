use shared::create_ui;
use slint::ComponentHandle;

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = create_ui();
    ui.run().unwrap();
}
