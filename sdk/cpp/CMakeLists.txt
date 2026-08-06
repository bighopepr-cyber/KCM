cmake_minimum_required(VERSION 3.14)
project(kcm-cpp-sdk VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# Basic example
add_executable(kcm_basic examples/basic/basic.cpp)
target_link_libraries(kcm_basic PRIVATE kcm)

# Tests
add_executable(kcm_test tests/test_kcm.cpp)
target_link_libraries(kcm_test PRIVATE kcm)

enable_testing()
add_test(NAME kcm_cpp_basic COMMAND kcm_basic)
add_test(NAME kcm_cpp_test COMMAND kcm_test)
