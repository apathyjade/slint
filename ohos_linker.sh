#!/bin/bash

# 过滤掉 OpenHarmony 链接器不支持的选项
args=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -Wl,--as-needed|-Wl,--no-as-needed)
            # 忽略这些选项
            ;;
        -Wl,-Bstatic|-Wl,-Bdynamic)
            # 忽略这些选项
            ;;
        -Wl,--eh-frame-hdr)
            # 忽略这些选项
            ;;
        -Wl,-z,*)
            # 忽略所有 -z 选项
            ;;
        -Wl,-z)
            # 忽略 -z 选项和下一个参数
            shift
            ;;
        -Wl,--gc-sections|--gc-sections)
            # 忽略这些选项
            ;;
        -pie)
            # 忽略这个选项
            ;;
        -Wl,-O1|-Wl,--strip-debug)
            # 忽略这些选项
            ;;
        -Wl,--*)
            # 忽略其他不支持的 -Wl 选项
            ;;
        -ltime_service_ndk|-lunwind)
            # 保留这些库，但稍后处理
            ;;
        -Wl,-ltime_service_ndk|-Wl,-lunwind)
            # 忽略这些库的-Wl形式
            ;;
        -nodefaultlibs)
            # 忽略这个选项
            ;;
        -m64)
            # 忽略这个选项（ld.lld 不需要）
            ;;
        --target=*|-target=*)
            # 忽略目标参数（clang++ 会处理）
            ;;
        *)
            # 添加其他参数
            args+=("$1")
            ;;
    esac
    shift
done

# 添加必要的库路径和库
args+=("--sysroot=/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot")
args+=("-L/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos")
args+=("-lc")
args+=("-ldl")

# 检查并添加时间服务库（如果存在）
if [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libtime_service_ndk.so" ]; then
    args+=("-ltime_service_ndk")
elif [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libtime_service_ndk.a" ]; then
    args+=("-ltime_service_ndk")
fi

# 检查并添加 unwind 库（如果存在）
if [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libunwind.so" ]; then
    args+=("-lunwind")
elif [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libunwind.a" ]; then
    args+=("-lunwind")
fi

# 尝试添加可能的 C++ 库
if [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libc++.so" ] || \
   [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libc++.a" ]; then
    args+=("-lc++")
fi

if [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libc++abi.so" ] || \
   [ -f "/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos/libc++abi.a" ]; then
    args+=("-lc++abi")
fi

# 调用实际的链接器
exec /Users/jade/Library/OpenHarmony/Sdk/20/native/llvm/bin/clang++ \
    --target=x86_64-unknown-linux-ohos \
    --sysroot=/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot \
    -L/Users/jade/Library/OpenHarmony/Sdk/20/native/sysroot/usr/lib/x86_64-linux-ohos \
    -lc -ldl \
    "${args[@]}"