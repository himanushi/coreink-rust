cmake_minimum_required(VERSION 3.5)
include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(your_project_name)

# Enable C++ support
set(CMAKE_CXX_STANDARD 14)

# Define ESP_PLATFORM
add_definitions(-DESP_PLATFORM)

# Include ESP-IDF components
include($ENV{IDF_PATH}/tools/cmake/idf.cmake)

# Add M5GFX and M5Unified libraries
add_subdirectory(clibs/M5GFX)
add_subdirectory(clibs/M5Unified)

# Link libraries
target_link_libraries(${PROJECT_NAME} PRIVATE M5GFX M5Unified)
