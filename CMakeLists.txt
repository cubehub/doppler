cmake_minimum_required(VERSION 2.8.0 FATAL_ERROR)

project(doppler)

add_executable(doppler doppler.c)

install(TARGETS doppler DESTINATION bin)
