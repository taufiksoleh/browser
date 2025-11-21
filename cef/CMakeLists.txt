cmake_minimum_required(VERSION 3.19)
project(cef_browser VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# CEF configuration
set(CEF_VERSION "120.1.10+g3ce3184+chromium-120.0.6099.129" CACHE STRING "CEF version to download")

# Platform detection
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    set(CEF_PLATFORM "linux64")
    set(CEF_DISTRIBUTION "cef_binary_${CEF_VERSION}_${CEF_PLATFORM}")
elseif(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    if(CMAKE_OSX_ARCHITECTURES STREQUAL "arm64" OR CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "arm64")
        set(CEF_PLATFORM "macosarm64")
    else()
        set(CEF_PLATFORM "macosx64")
    endif()
    set(CEF_DISTRIBUTION "cef_binary_${CEF_VERSION}_${CEF_PLATFORM}")
elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    if(CMAKE_SIZEOF_VOID_P EQUAL 8)
        set(CEF_PLATFORM "windows64")
    else()
        set(CEF_PLATFORM "windows32")
    endif()
    set(CEF_DISTRIBUTION "cef_binary_${CEF_VERSION}_${CEF_PLATFORM}")
endif()

# CEF download and setup
set(CEF_ROOT "${CMAKE_CURRENT_BINARY_DIR}/cef")
set(CEF_DOWNLOAD_URL "https://cef-builds.spotifycdn.com/${CEF_DISTRIBUTION}_minimal.tar.bz2")

if(NOT EXISTS "${CEF_ROOT}")
    message(STATUS "Downloading CEF from ${CEF_DOWNLOAD_URL}")
    file(DOWNLOAD
        "${CEF_DOWNLOAD_URL}"
        "${CMAKE_CURRENT_BINARY_DIR}/cef.tar.bz2"
        SHOW_PROGRESS
        STATUS DOWNLOAD_STATUS
    )
    list(GET DOWNLOAD_STATUS 0 STATUS_CODE)
    if(NOT STATUS_CODE EQUAL 0)
        message(FATAL_ERROR "Failed to download CEF. Please download manually from https://cef-builds.spotifycdn.com/")
    endif()

    message(STATUS "Extracting CEF...")
    execute_process(
        COMMAND ${CMAKE_COMMAND} -E tar xjf "${CMAKE_CURRENT_BINARY_DIR}/cef.tar.bz2"
        WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
    )
    file(RENAME "${CMAKE_CURRENT_BINARY_DIR}/${CEF_DISTRIBUTION}" "${CEF_ROOT}")
endif()

# Include CEF cmake modules
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CEF_ROOT}/cmake")
find_package(CEF REQUIRED)

# CEF wrapper library
add_subdirectory(${CEF_ROOT}/libcef_dll libcef_dll_wrapper)

# Source files
set(BROWSER_SOURCES
    src/main.cpp
    src/app.cpp
    src/app.h
    src/browser_client.cpp
    src/browser_client.h
    src/browser_window.cpp
    src/browser_window.h
    src/resource_util.cpp
    src/resource_util.h
)

# Main browser executable
add_executable(${PROJECT_NAME} WIN32 MACOSX_BUNDLE ${BROWSER_SOURCES})

target_include_directories(${PROJECT_NAME} PRIVATE
    ${CEF_ROOT}
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_link_libraries(${PROJECT_NAME} PRIVATE
    libcef_dll_wrapper
    ${CEF_STANDARD_LIBS}
)

# Copy CEF resources and binaries
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    # Linux: Copy libcef.so and resources
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_directory
            "${CEF_ROOT}/Release"
            "$<TARGET_FILE_DIR:${PROJECT_NAME}>"
        COMMAND ${CMAKE_COMMAND} -E copy_directory
            "${CEF_ROOT}/Resources"
            "$<TARGET_FILE_DIR:${PROJECT_NAME}>"
    )

    # Set rpath for Linux
    set_target_properties(${PROJECT_NAME} PROPERTIES
        BUILD_RPATH "$ORIGIN"
        INSTALL_RPATH "$ORIGIN"
    )

elseif(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    # macOS: Create app bundle structure
    set_target_properties(${PROJECT_NAME} PROPERTIES
        MACOSX_BUNDLE TRUE
        MACOSX_BUNDLE_INFO_PLIST "${CMAKE_CURRENT_SOURCE_DIR}/resources/macos/Info.plist"
        MACOSX_BUNDLE_BUNDLE_NAME "CEF Browser"
        MACOSX_BUNDLE_BUNDLE_VERSION "${PROJECT_VERSION}"
        MACOSX_BUNDLE_SHORT_VERSION_STRING "${PROJECT_VERSION}"
        MACOSX_BUNDLE_GUI_IDENTIFIER "com.browser.cef"
    )

    # Copy CEF framework
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_directory
            "${CEF_ROOT}/Release/Chromium Embedded Framework.framework"
            "$<TARGET_BUNDLE_DIR:${PROJECT_NAME}>/Contents/Frameworks/Chromium Embedded Framework.framework"
    )

elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    # Windows: Copy DLLs and resources
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_directory
            "${CEF_ROOT}/Release"
            "$<TARGET_FILE_DIR:${PROJECT_NAME}>"
        COMMAND ${CMAKE_COMMAND} -E copy_directory
            "${CEF_ROOT}/Resources"
            "$<TARGET_FILE_DIR:${PROJECT_NAME}>"
    )
endif()

# Helper executable for subprocess (required by CEF)
add_executable(${PROJECT_NAME}_helper
    src/helper_main.cpp
    src/app.cpp
    src/app.h
)

target_include_directories(${PROJECT_NAME}_helper PRIVATE
    ${CEF_ROOT}
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_link_libraries(${PROJECT_NAME}_helper PRIVATE
    libcef_dll_wrapper
    ${CEF_STANDARD_LIBS}
)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    add_custom_command(TARGET ${PROJECT_NAME}_helper POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
            "$<TARGET_FILE:${PROJECT_NAME}_helper>"
            "$<TARGET_FILE_DIR:${PROJECT_NAME}>"
    )
    set_target_properties(${PROJECT_NAME}_helper PROPERTIES
        BUILD_RPATH "$ORIGIN"
        INSTALL_RPATH "$ORIGIN"
    )
endif()

# Installation
install(TARGETS ${PROJECT_NAME}
    BUNDLE DESTINATION .
    RUNTIME DESTINATION bin
)

# ============================================================================
# Testing Support
# ============================================================================
option(BUILD_TESTS "Build unit tests" OFF)

if(BUILD_TESTS)
    enable_testing()

    # Find Google Test
    find_package(GTest QUIET)

    if(GTest_FOUND)
        message(STATUS "Google Test found - building tests")

        # Unit tests executable
        add_executable(${PROJECT_NAME}_tests
            tests/test_resource_util.cpp
        )

        target_include_directories(${PROJECT_NAME}_tests PRIVATE
            ${CMAKE_CURRENT_SOURCE_DIR}/src
            ${GTEST_INCLUDE_DIRS}
        )

        target_link_libraries(${PROJECT_NAME}_tests PRIVATE
            GTest::gtest
            GTest::gtest_main
        )

        # Add test to CTest
        add_test(NAME UnitTests COMMAND ${PROJECT_NAME}_tests)

        # Set test properties
        set_tests_properties(UnitTests PROPERTIES
            TIMEOUT 60
            LABELS "unit"
        )
    else()
        message(STATUS "Google Test not found - skipping unit tests")
        message(STATUS "  Install with: sudo apt-get install libgtest-dev")
    endif()
endif()

message(STATUS "CEF Browser configuration complete")
message(STATUS "  Platform: ${CEF_PLATFORM}")
message(STATUS "  CEF Version: ${CEF_VERSION}")
message(STATUS "  Build Tests: ${BUILD_TESTS}")
