cmake_minimum_required(VERSION 3.23)
project(linenoise)

set(CMAKE_CXX_STANDARD 20)

add_library(${PROJECT_NAME} SHARED linenoise.c linenoise.h)
