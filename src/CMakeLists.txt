CMAKE_MINIMUM_REQUIRED(VERSION 3.20)

SET(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_STANDARD_REQUIRED ON)

PROJECT(SuperSpaceNeo)

ADD_EXECUTABLE( SuperSpaceNeo
        SuperSpaceNeo.cpp
)

set(RUST_PROJECT_DIR ${CMAKE_SOURCE_DIR}/rust)

set(rust_fileFunc_DIR ${RUST_PROJECT_DIR}/rust_fileFunc)

set(rust_fileFunc_project_DIR ${rust_fileFunc_DIR}/src)

set(rust_fileFunc_build_DIR ${rust_fileFunc_DIR}/target/release)

ADD_CUSTOM_TARGET(rust_fileFunc_build
        COMMAND cargo build --release
        WORKING_DIRECTORY ${rust_fileFunc_DIR}
)

ADD_DEPENDENCIES(SuperSpaceNeo rust_fileFunc_build)

if(UNIX)
        set(rust_fileFunc_name "librust_fileFuc.a")
else()
        set(rust_fileFunc_name "rust_fileFunc.lib")
endif()

add_library(rust_ffi STATIC IMPORTED)

set_target_properties(rust_ffi PROPERTIES
	IMPORTED_LOCATION "${rust_fileFunc_build_DIR}/${rust_fileFunc_lib_name}"
)

link_directories(${rust_fileFunc_build_DIR})

target_link_libraries(SuperSpaceNeo PRIVATE rust_ffi)

ADD_SUBDIRECTORY(${CMAKE_SOURCE_DIR}/test)

TARGET_LINK_LIBRARIES(SuperSpaceNeo PRIVATE testSSN rust_ffi)
