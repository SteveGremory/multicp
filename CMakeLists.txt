cmake_minimum_required(VERSION 3.22)
project(MCP)

# Set C++ options
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(mcp source/main.cc source/mcp.cc)
target_include_directories(mcp PRIVATE source/)