#
# CMake file for generating the splashkit core library
#
cmake_minimum_required(VERSION 3.2)
project(splashkit)

# SK Directories relative to cmake project
set(SK_ROOT "../../../..")
set(SK_SRC "${SK_ROOT}/coresdk/src")
set(SK_EXT "${SK_ROOT}/coresdk/external")
set(SK_LIB "${SK_ROOT}/coresdk/lib")

set(SK_GENERATED "${SK_ROOT}/generated")
set(SK_BIN "${SK_ROOT}/bin")
set(SK_OUT "${SK_ROOT}/out")

set(SK_DEPLOY_ROOT "${SK_OUT}/splashkit-linux")
find_package(PNG REQUIRED)
include_directories(${PNG_INCLUDE_DIR})    

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

# SOURCE FILES
file(GLOB SOURCE_FILES
    "*.c"
    "*.cpp"
)

# FLAGS
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++14")

# SKSDK FILE INCLUDES
file(GLOB INCLUDE_FILES
    "../include/*.h"
)

# DIRECTORY INCLUDES
include_directories("../include")

# MACRO DEFINITIONS #
add_definitions(-DELPP_THREAD_SAFE)

#### END SETUP ####

#### SplashKitBackend STATIC LIBRARY ####


add_library(SplashKit SHARED ${SOURCE_FILES} ${SOURCE_FILES} ${INCLUDE_FILES})
target_link_libraries(SplashKit ${LIB_FLAGS})
target_link_libraries(SplashKit ${PNG_LIBRARY})

install(TARGETS SplashKit DESTINATION ~/.splashkit/lib)
