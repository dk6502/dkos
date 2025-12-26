cmake_minimum_required(VERSION 3.31)
include(FetchContent)

set(TOOLCHAIN "gcc")
set(CMAKE_TOOLCHAIN_FILE ${CMAKE_CURRENT_SOURCE_DIR/cmake/${TOOLCHAIN}.cmake})
project(kernel)


FetchContent_Declare(
  limine-protocol
  GIT_REPOSITORY https://codeberg.org/Limine/limine-protocol.git
  GIT_TAG trunk
)
FetchContent_MakeAvailable(limine-protocol)

FetchContent_Declare(
  freestnd-cxx-hdrs
  GIT_REPOSITORY     https://codeberg.org/OSDev/freestnd-cxx-hdrs.git
  GIT_TAG trunk
)
FetchContent_MakeAvailable(freestnd-cxx-hdrs)

add_executable(kernel src/main.cc)

target_include_directories(kernel PUBLIC
    ${limine-protocol_SOURCE_DIR}/include
    ${freestnd-cxx-hdrs_SOURCE_DIR}/${CMAKE_SYSTEM_PROCESSOR}/include
)


target_compile_options(kernel PUBLIC $<$<COMPILE_LANGUAGE:C,CXX>:
    -Wall
    -Wextra
>)

target_link_options(kernel PUBLIC
    -T${CMAKE_CURRENT_SOURCE_DIR}/ld/linker-x86_64.ld
    -nostdlib
    -zmax-page-size=0x1000
    -static
)

target_include_directories(kernel PUBLIC ${})
