cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)

project(Template 
    VERSION 0.1.0 
    LANGUAGES C CXX
)

set(CMAKE_CXX_STANDARD 17)

if(UNIX)
  # Can't compile Cinder under linux because linux is macro set to 1
  add_definitions(-Ulinux)
endif(UNIX)

# Fetch CPM.cmake
if(NOT CPM_SOURCE)
  set(CPM_SOURCE "${CMAKE_BINARY_DIR}/cmake/CPM.cmake")
  if(NOT EXISTS ${CPM_SOURCE})
    message(STATUS "Downloading CPM.cmake...")
    file(DOWNLOAD
      https://github.com/cpm-cmake/CPM.cmake/releases/latest/download/CPM.cmake
      ${CPM_SOURCE}
    )
  endif()
endif()

include(${CPM_SOURCE})

CPMAddPackage(
  NAME Cinder
  GITHUB_REPOSITORY cinder/Cinder
  GIT_TAG master
)

add_executable(${PROJECT_NAME}
    source/main.cpp
)

target_link_libraries(${PROJECT_NAME} PRIVATE cinder)
