cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)

project(RotatingCube 
    VERSION 0.1.0 
    LANGUAGES C CXX
)

set(CMAKE_CXX_STANDARD 17)

include(cmake/get_cpm.cmake)
include(cmake/get_cinder.cmake)

add_executable(${PROJECT_NAME}
    source/main.cpp
)

include(cmake/copy_shaders.cmake)

target_link_libraries(${PROJECT_NAME}  PRIVATE cinder)
