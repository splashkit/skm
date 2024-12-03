# CMake file for generating the splashkit core library

cmake_minimum_required(VERSION 3.5)
project(SplashKit)

# Detect Windows and flag MSYS
if (WIN32 OR MSYS OR MINGW)
    message(SEND_ERROR "Source compile only available for Linux")
    return ()
elseif (APPLE)
    set(PATH_SUFFIX "macos")
else()
    set(PATH_SUFFIX "linux")

    # Detect if the platform is Raspberry Pi
    set(RASPBERRY_PI FALSE)

    # Method 1: Check for Raspberry Pi specific file
    if(EXISTS "/proc/device-tree/model")
        file(READ "/proc/device-tree/model" DEVICE_MODEL)
        if(DEVICE_MODEL MATCHES "Raspberry Pi")
            set(RASPBERRY_PI TRUE)
        endif()
    endif()

    # Method 2: Check for ARM architecture and Broadcom chip
    if(CMAKE_SYSTEM_PROCESSOR MATCHES "^(arm|aarch64)")
        if(EXISTS "/proc/cpuinfo")
            file(READ "/proc/cpuinfo" CPU_INFO)
            if(CPU_INFO MATCHES "BCM[0-9]+")
                set(RASPBERRY_PI TRUE)
            endif()
        endif()
    endif()

    if(RASPBERRY_PI)
        message("-- Raspberry Pi Detected")

        # RASPBERRY PI SETUP
        # Locate Pigpio library
        find_path(PIGPIOD_INCLUDE_DIR NAMES pigpiod_if2.h PATHS "/usr/include")
        find_library(PIGPIOD_IF2_LIB NAMES pigpiod_if2 HINTS "/usr/lib")
        find_library(PIGPIO_LIB NAMES pigpio HINTS "/usr/lib")

        # Handle Pigpio library finding
        include(FindPackageHandleStandardArgs)
        find_package_handle_standard_args(PIGPIO DEFAULT_MSG PIGPIOD_INCLUDE_DIR PIGPIOD_IF2_LIB PIGPIO_LIB)

        if(PIGPIO_FOUND)
            message("-- Pigpio libraries found")
            include_directories(${PIGPIOD_INCLUDE_DIR})
            add_definitions(-DRASPBERRY_PI)
        else()
            message(WARNING "-- Pigpio libraries NOT found. Some features may be unavailable.")
        endif()
    endif()
endif()

# SK Directories relative to cmake project
set(SKM_ROOT "..")
get_filename_component(SKM_ROOT ${SKM_ROOT} ABSOLUTE) # Convert root to absolute path

# SK_SRC is the source folder
set(SK_SRC "${SKM_ROOT}/source")
# SK_LIB for the output library location
set(SK_LIB "${SKM_ROOT}/lib/${PATH_SUFFIX}")
set(SK_DEPLOY_ROOT "${SK_LIB}")
# SK_GPP for the g++ adapter files
set(SK_GPP "${SKM_ROOT}/g++")

# Setup requirements using PkgConfig
find_package(PkgConfig REQUIRED)
pkg_check_modules(SPLASHKIT_REQUIRED_LIBS REQUIRED sdl2 SDL2_ttf SDL2_image SDL2_net SDL2_mixer SDL2_gfx libpng libcurl)

# Setup include paths
include_directories("${SK_SRC}/include")
include_directories("${SK_GPP}/include")
include_directories(${SPLASHKIT_REQUIRED_LIBS_INCLUDE_DIRS})

# FLAGS
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17 -fPIC")

# MACRO DEFINITIONS #
add_definitions(-DELPP_THREAD_SAFE)

#### END SETUP ####

# SOURCE FILES
file(GLOB C_SOURCE_FILES
"${SK_SRC}/*.c"
)

# SOURCE FILES
file(GLOB SOURCE_FILES
    "${SK_SRC}/*.cpp"
    "${SK_SRC}/easylogging++.cc"
)

# Adapter SOURCE
file(GLOB SPLASHKITCPP_SOURCE_FILES
  "${SK_GPP}/src/*.cpp"
)

#### SplashKit SHARED LIBRARY ####
add_library(SplashKit SHARED ${SOURCE_FILES} ${C_SOURCE_FILES} ${SPLASHKITCPP_SOURCE_FILES})

# Link libraries based on platform
if (RASPBERRY_PI AND PIGPIO_FOUND)
    target_link_libraries(SplashKit ${SPLASHKIT_REQUIRED_LIBS_LDFLAGS} ${PIGPIOD_IF2_LIB})
else()
    target_link_libraries(SplashKit ${SPLASHKIT_REQUIRED_LIBS_LDFLAGS})
endif()

if (APPLE)
    target_link_libraries(SplashKit "-framework CoreFoundation")
endif()

install(TARGETS SplashKit DESTINATION ${SK_DEPLOY_ROOT})
