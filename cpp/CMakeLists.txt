cmake_minimum_required (VERSION 3.1)

project(lorentz LANGUAGES CXX VERSION 1.0.0)

set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "-Wall -Wextra -pedantic")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
set(CMAKE_BUILD_TYPE "Release")

# Add march=native?
include(CheckCXXCompilerFlag)
CHECK_CXX_COMPILER_FLAG("-march=native" COMPILER_SUPPORTS_MARCH_NATIVE)
if(COMPILER_SUPPORTS_MARCH_NATIVE)
  set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -march=native")
  message("Compiler option march=native enabled.")
endif()

project(lorentz)

find_package(Threads REQUIRED)

set(TheFiles
  src/vec3.h
  src/vec3.cpp
  src/random.h
  src/random.cpp
  src/ray.h
  src/material.h
  src/material.cpp
  src/utils.h
  src/config.h
  src/primitive_trait.h
  src/primitive_trait.cpp
  src/primitive.h
  src/primitive.cpp
  src/bvh.h
  src/bvh.cpp
  src/camera.h
  src/camera.cpp
  )

add_executable(lorentz src/main.cpp ${TheFiles})
target_link_libraries(lorentz ${CMAKE_THREAD_LIBS_INIT})

# set(TestFiles
#   test/test-xtcio.cpp
#   test/test-pbc.cpp
#   test/test-sdf.cpp
#   test/testutils.h
#   )

# IF(Catch2_FOUND)
#   add_executable(mdtest test/main.cpp ${TestFiles} ${TheFiles})
#   target_link_libraries(mdtest Eigen3::Eigen Catch2::Catch2 ${CMAKE_THREAD_LIBS_INIT})
#   target_include_directories(mdtest PRIVATE src)
#   include(CTest)
#   include(Catch)
#   catch_discover_tests(mdtest)
# ELSE()
#   message(WARNING "Catch2 not found.")
# ENDIF()
