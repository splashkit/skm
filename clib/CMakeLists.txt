#
# CMake file for generating the splashkit core library
#
cmake_minimum_required(VERSION 3.2)
project(splashkit)

set(SK_OUT "..")

set(SK_DEPLOY_ROOT "${SK_OUT}")
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
               -lpng12 \
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

# SOURCE FILES
file(GLOB SOURCE_FILES
    "./*.cpp"
    "./*.c"
)

# SKSDK FILE INCLUDES
file(GLOB INCLUDE_FILES
    "./*.h"
)

# DIRECTORY INCLUDES
include_directories(".")

# MACRO DEFINITIONS #
add_definitions(-DELPP_THREAD_SAFE)

#### END SETUP ####

#### SplashKitBackend STATIC LIBRARY ####
add_library(SplashKit SHARED ${SOURCE_FILES} ${INCLUDE_FILES})
target_link_libraries(SplashKit ${LIB_FLAGS})

get_filename_component(SK_DEPLOY_ROOT ${SK_DEPLOY_ROOT} ABSOLUTE)

install(TARGETS SplashKit DESTINATION ${SK_DEPLOY_ROOT}/lib)
# install(FILES ${SK_LIB_INCLUDE_FILE} DESTINATION ${SK_DEPLOY_ROOT}/include)
