fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    // 添加重新运行条件，当UI文件或构建脚本改变时重新运行
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=ui/main.slint");
    println!("cargo:rerun-if-changed=build.rs");
    
    eprintln!("Starting Slint compilation...");
    eprintln!("Target OS: {}", os);
    
    let mut config = slint_build::CompilerConfiguration::new();
    
    // 根据目标平台设置不同的配置
    match os.as_str() {
        "harmony" | "ohos" => {
            eprintln!("Compiling for HarmonyOS...");
            // HarmonyOS特定配置
            config = config.with_style("fluent".into());
        }
        _ => {
            config = config.with_style("fluent".into());
        }
    }
    
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