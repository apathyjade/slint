
// #![cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
// #![windows_subsystem = "windows"]

mod ui;

fn main() {
    ui::create_ui();
}
