cmake_minimum_required(VERSION 3.4.1)
project(MyApp)

# 添加你的 Rust 库
add_library(app SHARED IMPORTED)
set_target_properties(app PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/libapp.so)

# 链接系统库
target_link_libraries(app)