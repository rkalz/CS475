cmake_minimum_required(VERSION 3.13)
project(bilinear_interp)

set(CMAKE_CXX_STANDARD 17)

add_definitions(-DPROJECT_DIR="${CMAKE_SOURCE_DIR}")

add_executable(bilinear_interp main.cc)