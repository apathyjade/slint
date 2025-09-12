
#![cfg(target_os = "windows")]
#![windows_subsystem = "windows"]

mod ui;

fn main() {
    ui::create_ui();
}
