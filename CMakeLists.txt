cmake_minimum_required(VERSION 3.24.2)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(Alstroemeria)

set(SOURCES ./src/main.cpp)

add_subdirectory(glfw)

find_package(Vulkan REQUIRED)

add_executable(Alstroemeria ${SOURCES})
target_link_libraries(Alstroemeria PUBLIC glfw)
