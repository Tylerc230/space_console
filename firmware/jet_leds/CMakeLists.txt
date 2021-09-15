# Define CMake's minimum version (must-do) and the project's name and supported languages
cmake_minimum_required(VERSION 3.8.2)
project(Jet_LEDs LANGUAGES C CXX ASM)
set( CMAKE_CXX_FLAGS "-DFASTLED_RGBW" )
# Call a framework utility function, passing it information about the hardware board that will be used - This function returns a structure known only to the framework
get_board_id(board_id pro 16MHzatmega328)
file(GLOB jet_LEDs_src
    "*.h"
    "*.cpp"
    "programs/*.h"
    "programs/*.cpp"
)
# Create an executable suitable for Arduino using CMake-style target-creation
add_arduino_executable(Jet_LEDs ${board_id} ${jet_LEDs_src})

find_arduino_library(fastled FastLED ${board_id} 3RD_PARTY)
link_arduino_library(Jet_LEDs fastled ${board_id})

find_arduino_library(encoder Encoder ${board_id} 3RD_PARTY)
link_arduino_library(Jet_LEDs encoder ${board_id})

target_include_directories(Jet_LEDs PRIVATE /Applications/Arduino.app/Contents/Java/hardware/tools/avr/avr/include)
target_include_directories(Jet_LEDs PRIVATE /Applications/Arduino.app/Contents/Java/hardware/arduino/avr/cores/arduino/)

#find_arduino_library(hardware_serial Serial ${board_id})
#link_arduino_library(Jet_LEDs hardware_serial ${board_id})
#link_platform_library(Jet_LEDs Serial ${board_id})
# Upload the created target through a connected Serial Port (Where your board is connected to)
upload_arduino_target(Jet_LEDs "${board_id}" /dev/cu.usbserial-A1014HUC)
