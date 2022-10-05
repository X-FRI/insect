cmake_minimum_required(VERSION 3.0.0)
project(sinbuger)

set(CMAKE_CXX_STANDARD 20)

add_subdirectory(lib/linenoise)
include_directories(lib/linenoise)
add_executable(${PROJECT_NAME} src/main.cpp src/debugger.cpp src/debugger.h src/breakpoint.cpp src/breakpoint.h)
target_link_libraries(${PROJECT_NAME} linenoise)