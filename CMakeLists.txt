cmake_minimum_required(VERSION 2.8.0 FATAL_ERROR)

project(doppler)

add_executable(doppler
	doppler.c
	predict.c
	dsp.c
	libgpredict/sgpsdp/sgp4sdp4.c
	libgpredict/sgpsdp/sgp_math.c
	libgpredict/sgpsdp/sgp_in.c
	libgpredict/sgpsdp/sgp_time.c
	libgpredict/sgpsdp/sgp_obs.c
)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")

# Glib
# cmake/FindGlib.cmake is taken from https://github.com/WebKit/webkit/blob/master/Source/cmake/FindGLIB.cmake
find_package(glib REQUIRED)
include_directories(${GLIB_INCLUDE_DIR})
include_directories(${GLIBCONFIG_INCLUDE_DIR})
target_link_libraries(doppler ${GLIB_LIBRARIES})

install(TARGETS doppler DESTINATION bin)
