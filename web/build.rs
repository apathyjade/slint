// web/build.rs
use std::process::Command;

fn main() {
  // 调用 wasm-pack 构建，参数与直接执行一致
  let status = Command::new("wasm-pack")
      .arg("build")
      .arg("--target")
      .arg("web")
      .arg("--out-dir")
      .arg("pkg")
      // 自动根据 cargo 构建模式（debug/release）选择
      .arg(if std::env::var("PROFILE").unwrap() == "release" {
          "--release"
      } else {
          "--dev"
      })
      .status()
      .expect("无法执行 wasm-pack，请确保已安装（cargo install wasm-pack）");

  if !status.success() {
      panic!("wasm-pack 构建失败");
  }
}