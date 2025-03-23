fn main() {
  slint_build::compile("ui/view.slint").unwrap();
  slint_build::compile("ui/main.slint").unwrap();
}