use shared::create_ui;
use slint::ComponentHandle;
 fn main() {
    let ui = create_ui();
    ui.run().unwrap();
}
