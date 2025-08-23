use std::process::Command;
use std::env;

fn main() {
    // 设置环境变量以便在开发时重新运行
    // println!("cargo:rerun-if-env-changed=PROFILE");
    if env::var("--target").unwrap_or("".to_string()).starts_with("wasm") {
        return
    }
    
    // 检查 wasm-pack 是否存在
    if let Err(_) = Command::new("wasm-pack").arg("--version").output() {
        eprintln!("错误: 未找到 wasm-pack 命令");
        eprintln!("请安装 wasm-pack: cargo install wasm-pack");
        std::process::exit(1);
    }
    
    // 显示当前工作目录
    let current_dir = env::current_dir().unwrap();
    println!("当前工作目录: {:?}", current_dir);
    
    // 调用 wasm-pack 构建，参数与直接执行一致
    let mut cmd = Command::new("wasm-pack");
    cmd.arg("build")
       .arg("--target")
       .arg("web")
       .arg("--out-dir")
       .arg("pkg");
    //    .arg("--mode")
    //    .arg("no-install");

    // 安全地获取构建配置
    // let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    // let profile_arg = if profile == "release" {
    //     "--release"
    // } else {
    //     "--dev"
    // };
    // cmd.arg(profile_arg);
    
    // 添加更多日志信息
    // println!("正在构建，配置: {:?}, 参数: {:?}", profile, profile_arg);
    println!("执行命令: {:?}", cmd);
    
    // 检查目标目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("Cargo manifest 目录: {}", manifest_dir);
    
    // 执行命令并处理结果
    match cmd.output() {
        Ok(output) => {
            // 打印 stdout 和 stderr 以便调试
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            
            if !output.status.success() {
                eprintln!("wasm-pack 构建失败，退出码: {:?}", output.status.code());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("无法执行 wasm-pack: {}", e);
            eprintln!("请确保已安装 wasm-pack: cargo install wasm-pack");
            std::process::exit(1);
        }
    }
    
    println!("wasm-pack 构建完成");
}