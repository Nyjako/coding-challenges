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
