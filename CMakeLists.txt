cmake_minimum_required(VERSION 4.1)
project(SuperSpaceNeo)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)

add_executable(SuperSpaceNeo
        SuperSpaceNeo.cpp
        SuperSpaceNeo.h
        src/main/main.cpp
        src/main/main.h)
