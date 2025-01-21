project(cxx)
cmake_minimum_required(VERSION 3.0)

# Find the compiler and set dialect
message(STATUS "Found Compiler: ${CMAKE_CXX_COMPILER} (found version \"${CMAKE_CXX_COMPILER_VERSION}\")")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17")

if(CMAKE_COMPILER_IS_GNUCXX)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -fPIC -pipe")

    # Determine if binaries should be stripped
    if(release)
        message(STATUS "Strip binaries on link step")
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -s")
    endif(release)
endif()

set(CMAKE_INCLUDE_CURRENT_DIR ON)
include_directories(${CMAKE_SOURCE_DIR})

# Setup Google Test/Mock
enable_testing()
include( CTest )
message (STATUS "gtest libraries " ${GTEST_LIBRARIES})
message (STATUS "gmock libraries " ${GMOCK_LIBRARIES})

add_subdirectory(withdraw)
