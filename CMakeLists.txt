cmake_minimum_required(VERSION 3.13) # CMake version check
project(backstage)
set(CMAKE_CXX_STANDARD 20)           # Enable C++20 standard

add_executable(backstage src/main.cpp)

set(DAWN_FETCH_DEPENDENCIES ON)
add_subdirectory("dawn" EXCLUDE_FROM_ALL)
target_link_libraries(app PRIVATE webgpu_cpp webgpu_dawn webgpu_glfw)