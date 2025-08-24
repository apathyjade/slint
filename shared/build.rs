fn main() {
    // 添加重新运行条件，当UI文件或构建脚本改变时重新运行
    println!("cargo:rerun-if-changed=ui/main.slint");
    println!("cargo:rerun-if-changed=build.rs");
    eprintln!("Starting Slint compilation...");
    let config = slint_build::CompilerConfiguration::new().with_style("fluent".into());
    eprintln!("Compiling ui/main.slint...");
    slint_build::compile_with_config("ui/main.slint", config).unwrap_or_else(|e| {
        eprintln!("Failed to compile Slint UI file: {}", e);
        eprintln!("Current directory: {:?}", std::env::current_dir());
        eprintln!(
            "UI file exists: {:?}",
            std::path::Path::new("ui/main.slint").exists()
        );
        std::process::exit(1);
    });
}
