cmake_minimum_required(VERSION 3.16)

project(QCatppuccinDecorations
    DESCRIPTION "Qt Wayland plugin implementing Catppuccin-like client-side decorations"
    LANGUAGES CXX C)

option(USE_QT6 "Use Qt6 instead of Qt5" OFF)

set(CMAKE_AUTOMOC ON)

if (USE_QT6)
    set(QT_MIN_VERSION "6.5.0")
    set(CMAKE_CXX_STANDARD 17)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)
else()
    set(QT_MIN_VERSION "5.15.2")
    set(CMAKE_CXX_STANDARD 14)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)
endif()

include(GNUInstallDirs)
include(FeatureSummary)

# Qt version-specific setup
if (USE_QT6)
    find_package(Qt6 ${QT_MIN_VERSION} REQUIRED COMPONENTS
        Core
        Gui
        Svg
        WaylandClient
        Widgets
        GuiPrivate
        WaylandClientPrivate
    )
else()
    find_package(Qt5 ${QT_MIN_VERSION} REQUIRED COMPONENTS
        Core
        Gui
        Svg
        WaylandClient
        Widgets
    )

    find_package(Qt5Gui ${QT_MIN_VERSION} CONFIG REQUIRED Private)
    find_package(Qt5WaylandClient ${QT_MIN_VERSION} CONFIG REQUIRED Private)
    find_package(Qt5ThemeSupport REQUIRED)
    find_package(Qt5XkbCommonSupport ${QT_MIN_VERSION})
endif()

# Optional feature flag
if (HAS_QT6_SUPPORT OR USE_QT6)
    message(STATUS "Enabling Qt Wayland decoration shadows support")
    message(STATUS "NOTE: This support requires changes in Qt Wayland from Qt 6.2")
    add_definitions(-DHAS_QT6_SUPPORT)
endif()

# Determine plugin install path
if (NOT QT_PLUGINS_DIR)
    if (NOT USE_QT6)
        get_target_property(REAL_QMAKE_EXECUTABLE Qt5Core_QMAKE_EXECUTABLE IMPORTED_LOCATION)
        execute_process(COMMAND "${REAL_QMAKE_EXECUTABLE}" -query QT_INSTALL_PLUGINS
                        OUTPUT_VARIABLE QT_PLUGINS_DIR
                        ERROR_QUIET OUTPUT_STRIP_TRAILING_WHITESPACE)
    else()
        set(QT_PLUGINS_DIR ${Qt6_INSTALL_PLUGINS})
    endif()
endif()

add_subdirectory(src)

feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)
