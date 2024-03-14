#
# CMake file for generating the splashkit core library
#

cmake_minimum_required(VERSION 3.5)
project(SplashKit)

# Detect Windows and flag MSYS
if (WIN32 OR MSYS OR MINGW)
    message(SEND_ERROR "Source compile only available for Linux")
    return ()
elseif (APPLE)
    set(PATH_SUFFIX "macos")
else  ( )
    set(PATH_SUFFIX "linux")
    SET(LINUX "true")
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
pkg_check_modules(SPLASHKIT_REQUIRED_LIBS REQUIRED sdl2 sdl2_ttf sdl2_image sdl2_net sdl2_mixer sdl2_gfx libpng libcurl)

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
add_library(SplashKit SHARED ${SOURCE_FILES} ${C_SOURCE_FILES} ${OS_SOURCE_FILES} ${INCLUDE_FILES} "${SPLASHKITCPP_SOURCE_FILES}")

target_link_libraries(SplashKit ${SPLASHKIT_REQUIRED_LIBS_LDFLAGS})
if (APPLE)
    target_link_libraries(SplashKit "-framework CoreFoundation")
endif()

install(TARGETS SplashKit DESTINATION ${SK_DEPLOY_ROOT})
