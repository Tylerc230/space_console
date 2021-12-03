# Define CMake's minimum version (must-do) and the project's name and supported languages
cmake_minimum_required(VERSION 3.8.2)
project(Jet_LEDs LANGUAGES C CXX ASM)
set( CMAKE_CXX_FLAGS "-DFASTLED_RGBW" )
file(GLOB jet_LEDs_src
    "src/*.h"
    "src/*.cpp"
    "src/programs/*.h"
    "src/programs/*.cpp"
)
add_executable(Jet_LEDs ${jet_LEDs_src})
target_link_arduino_libraries(Jet_LEDs PRIVATE core)
target_link_arduino_libraries(Jet_LEDs PRIVATE FastLED Encoder)
target_enable_arduino_upload(Jet_LEDs)
