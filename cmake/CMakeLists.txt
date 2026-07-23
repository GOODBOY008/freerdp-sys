# CMake helper for freerdp-sys vendored builds.
#
# This file is optional — the primary build path uses the `cmake` Rust crate
# which invokes FreeRDP's own CMakeLists.txt directly from vendor/FreeRDP.
#
# You can use this wrapper if you need to customize the build further,
# e.g., adding extra include paths or overriding compiler flags.
#
# Usage (standalone, for testing):
#   mkdir build && cd build
#   cmake .. -DFREERDP_SOURCE_DIR=../vendor/FreeRDP
#   cmake --build .

cmake_minimum_required(VERSION 3.13)
project(freerdp-sys-vendor C)

set(FREERDP_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../vendor/FreeRDP"
    CACHE PATH "Path to FreeRDP source tree")

if(NOT EXISTS "${FREERDP_SOURCE_DIR}/CMakeLists.txt")
    message(FATAL_ERROR
        "FreeRDP source not found at ${FREERDP_SOURCE_DIR}.\n"
        "Run: git submodule update --init --recursive")
endif()

# Force static library build for Rust FFI
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)
set(WITH_SERVER OFF CACHE BOOL "" FORCE)
set(WITH_CLIENT OFF CACHE BOOL "" FORCE)
set(WITH_CHANNELS OFF CACHE BOOL "" FORCE)
set(BUILD_TESTING OFF CACHE BOOL "" FORCE)
set(CMAKE_BUILD_TYPE Release CACHE STRING "" FORCE)

add_subdirectory("${FREERDP_SOURCE_DIR}" freerdp-build)
