cmake_minimum_required(VERSION 3.14)
project(kcm-cpp-sdk VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(kcm_example example.cpp)
target_include_directories(kcm_example PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(kcm_example PRIVATE kcm)

enable_testing()
add_test(NAME kcm_cpp_example COMMAND kcm_example)
