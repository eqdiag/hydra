cmake_minimum_required(VERSION 3.0...3.25)
project(
    learnWebgpu # name of the project, which will also be the name of the visual studio solution if you use it
    VERSION 0.1.0 # any version number
    LANGUAGES CXX C # programming languages used by the project
)

## get submodules from github
find_package(Git QUIET)
if(GIT_FOUND AND EXISTS "${PROJECT_SOURCE_DIR}/.git")
## Update submodules as needed
    option(GIT_SUBMODULE "Check submodules during build" ON)
    if(GIT_SUBMODULE)
        message(STATUS "Submodule update")
        execute_process(COMMAND ${GIT_EXECUTABLE} submodule update --init --recursive
                        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
                        RESULT_VARIABLE GIT_SUBMOD_RESULT)
        if(NOT GIT_SUBMOD_RESULT EQUAL "0")
            message(FATAL_ERROR "git submodule update --init failed with ${GIT_SUBMOD_RESULT}, please checkout submodules")
        endif()
    endif()
endif()

##   CHECK ALL THE SUBMODULES
if(NOT EXISTS "${PROJECT_SOURCE_DIR}/external/glfw/CMakeLists.txt")
    message(FATAL_ERROR "The glfw submodules was not downloaded! GIT_SUBMODULE was turned off or failed. Please update submodules and try again.")
endif()



file(GLOB src
    #"src/engine.cpp"
    #"src/core/*.cpp"
    #"src/math/*.cpp"
    #"src/widgets/*.cpp"
    #"external/stb_img/stb_image.cpp"
    #"external/tiny_obj/tiny_obj_loader.cpp"
)


#Set asset paths
set(ASSET_DIR 
	" \"${CMAKE_CURRENT_LIST_DIR}/assets/\" "
)

set(SHADER_DIR 
    " \"${CMAKE_CURRENT_LIST_DIR}/assets/shaders/\" "
)

set(MODEL_DIR 
    " \"${CMAKE_CURRENT_LIST_DIR}/assets/models/\" "
)

set(TEXTURE_DIR 
    " \"${CMAKE_CURRENT_LIST_DIR}/assets/textures/\" "
)

message("Using Asset directory: ${ASSET_DIR}")
message("Using Model directory: ${MODEL_DIR}")
message("Using Texture directory: ${TEXTURE_DIR}")
message("Using Shader directory: ${SHADER_DIR}")



configure_file(
    "${CMAKE_CURRENT_LIST_DIR}/src/core/settings.h.in"
    "${CMAKE_CURRENT_LIST_DIR}/src/core/settings.h"
)


message("Building learnWebgpu...")


add_subdirectory(external/glfw)
add_subdirectory(external/webgpu)
add_subdirectory(external/glfw3webgpu)


add_executable (
    ${PROJECT_NAME}
    src/main.cpp
    #${src}
    #${imgui_src}
)

target_include_directories(
    ${PROJECT_NAME} 
    PUBLIC external/glfw/include
    PUBLIC external/webgpu/include
    PUBLIC external
    PUBLIC src
)

target_link_libraries(
    ${PROJECT_NAME} 
    PRIVATE
    glfw
    webgpu
    glfw3webgpu
)


set_target_properties(learnWebgpu PROPERTIES
    CXX_STANDARD 17
    CXX_EXTENSIONS OFF
    COMPILE_WARNING_AS_ERROR ON
    VS_DEBUGGER_ENVIRONMENT "DAWN_DEBUG_BREAK_ON_ERROR=1"
)





if (MSVC)
    target_compile_options(learnWebgpu PRIVATE /W4)
else()
    target_compile_options(learnWebgpu PRIVATE -Wall -Wextra -pedantic)
endif()

if(XCODE)
    set_target_properties(learnWebgpu PROPERTIES
        XCODE_GENERATE_SCHEME ON
        XCODE_SCHEME_ENABLE_GPU_FRAME_CAPTURE_MODE "Metal")
endif()