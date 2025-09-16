// entry/src/main/cpp/main.cpp
#include "native_layer.h"
#include <unistd.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdlib.h>

static int execute_rust_app() {
    // 直接执行你的 Rust 程序
    int result = execl("/data/local/tmp/app", "app", (char *)NULL);
    return result;
}

extern "C" OH_DLL_EXPORT int InitNativeLayer(NativeLayer *nativeLayer, NativeResourceManager *mgr) {
    // 在这里启动你的 Rust 程序
    // 可以选择立即执行或在特定事件时执行
    execute_rust_app();
    return 0;
}