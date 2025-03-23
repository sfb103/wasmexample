cmake_minimum_required(VERSION 3.5.1)
set(PROJECT_NAME_STR wasm-cpp)
project(${PROJECT_NAME_STR})
message(DEBUG "configuring ${PROJECT_NAME_STR} project")

# Define build options. These must be unset at the end of the file
option(release "release")  #-Drelease=ON for CMAKE, -Prelease=true for gradle
option(coverage "coverage")  #-Dcoverage=ON for CMAKE, -Pcoverage=true for gradle

# Setup the wasm32-wasip2 target
include(/wasi-sysroot/wasi-sdk-p2.cmake)
# Enables the WASI Reactor ABI
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -mexec-model=reactor")

# Find the compiler and set dialect
message(STATUS "Found Compiler: ${CMAKE_CXX_COMPILER} (found version \"${CMAKE_CXX_COMPILER_VERSION}\")")

# Setup Paths. Add CMAKE_CURRENT_SOURCE_DIR and CMAKE_CURRENT_BINARY_DIR
# to include path, enabling compiler to find generated include files
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}")
include_directories(${CMAKE_SOURCE_DIR})

# Generate C from our wit using wit-bindgen
set(WIT_FILES
        "../wit/infoxchange.wit")
set(WIT_BINDGEN_DIR wit_bindgen)
set(WIT_BINDGEN_FILES
        "${WIT_BINDGEN_DIR}/infoxchange.c"
        "${WIT_BINDGEN_DIR}/infoxchange_component_type.o")
add_custom_command(OUTPUT ${WIT_BINDGEN_FILES}
        DEPENDS ${WIT_FILES}
        COMMAND /.cargo/bin/wit-bindgen c ${WIT_FILES} --out-dir ${WIT_BINDGEN_DIR}
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
        COMMENT "Updating wit_bindgen files for infoxchange" VERBATIM)
add_custom_target(witBindgenInfoxchange
        DEPENDS ${WIT_BINDGEN_FILES} )

file(GLOB SOURCE_FILES
        "*.h"
        "*.cpp")
add_executable(infoxchange.wasm ${SOURCE_FILES} ${WIT_BINDGEN_FILES})
add_dependencies(infoxchange.wasm witBindgenInfoxchange)

# Unset our options.
# This is important, or else same settings used this build could be cached and unintentionally used next build
unset(release CACHE)
unset(coverage CACHE)