fn main() {
    // 添加重新运行条件，当UI文件或构建脚本改变时重新运行
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
