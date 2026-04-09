CMAKE_MINIMUM_REQUIRED(VERSION 3.20)

SET(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_STANDARD_REQUIRED ON)

PROJECT(SuperSpaceNeo)

ADD_EXECUTABLE( SuperSpaceNeo
        SuperSpaceNeo.cpp
        SuperSpaceNeo.cpp
        test/testSSN.cpp
)

SET(RUST_PROJECT_DIR ${CMAKE_SOURCE_DIR}/rust/rust_fileFunc)

SET(RUST_LIB_PATH ${RUST_PROJECT_DIR}/target/release)

ADD_CUSTOM_TARGET(rustlib_build
        COMMAND cargo build --release
        WORKING_DIRECTORY ${RUST_PROJECT_DIR}
)

ADD_DEPENDENCIES(SuperSpaceNeo rustlib_build)

TARGET_LINK_LIBRARIES( SuperSpaceNeo PRIVATE
        ${RUST_LIB_PATH}/rust_fileFunc.lib
)

ADD_SUBDIRECTORY(${CMAKE_SOURCE_DIR}/test)

TARGET_LINK_LIBRARIES(SuperSpaceNeo PRIVATE testSSN)