#
# CMake file for generating the splashkit core library
#

cmake_minimum_required(VERSION 3.2)
project(SplashKit)

# Detect Windows and flag MSYS
if (WIN32 OR MSYS OR MINGW)
    message(SEND_ERROR "Source compile only available for Linux")
    return ()
elseif (APPLE)
    message(SEND_ERROR "Source compile only available for Linux")
    return ()
else  ( )
    set(PATH_SUFFIX "linux")
endif()

# SK Directories relative to cmake project
set(SKM_ROOT "..")
set(SK_SRC "${SKM_ROOT}/source")
set(SK_LIB "${SKM_ROOT}/lib/${PATH_SUFFIX}")

set(SK_DEPLOY_ROOT "${SK_LIB}")

# SOURCE FILES
file(GLOB C_SOURCE_FILES
"${SK_SRC}/*.c"
)

file(GLOB CPP_SOURCE_FILES
"${SK_SRC}/*.cpp"
)

# SOURCE FILES
file(GLOB SOURCE_FILES
    "${SK_SRC}/*.cpp"
    "${SK_SRC}/easylogging++.cc"
)

# SKSDK FILE INCLUDES
include_directories("${SKM_ROOT}/source/include")

# LINUX PROJECT FLAGS
find_package(PNG REQUIRED)
find_package(CURL REQUIRED)
include_directories(${PNG_INCLUDE_DIR})
SET(LINUX "true")
set(LIB_FLAGS "-lSDL2 \
               -lSDL2_mixer \
               -lSDL2_ttf \
               -lSDL2_gfx \
               -lSDL2_image \
               -lSDL2_net \
               -lpthread \
               -lbz2 \
               -lFLAC \
               -lvorbis \
               -lz \
               -lvorbisfile \
               -lmikmod \
               -logg \
               -lwebp \
               -lfreetype \
               -lcurl \
               -lncurses \
               -ldl")

# FLAGS
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++14")

# MACRO DEFINITIONS #
add_definitions(-DELPP_THREAD_SAFE)

#### END SETUP ####

#### SplashKitBackend STATIC LIBRARY ####


add_library(SplashKit SHARED ${SOURCE_FILES} ${C_SOURCE_FILES} ${OS_SOURCE_FILES} ${INCLUDE_FILES})
target_link_libraries(SplashKit ${LIB_FLAGS})

target_link_libraries(SplashKit ${PNG_LIBRARY})

get_filename_component(SK_DEPLOY_ROOT ${SK_DEPLOY_ROOT} ABSOLUTE)

install(TARGETS SplashKit DESTINATION ${SK_DEPLOY_ROOT})

INSTALL(CODE "execute_process( \
   COMMAND ${CMAKE_COMMAND} -E create_symlink \
   libSplashKit.so \
   ${SK_DEPLOY_ROOT}/libsplashkit.dll   \
   )"
)
