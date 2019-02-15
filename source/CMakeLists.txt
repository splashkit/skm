#
# CMake file for generating the splashkit core library
#

if (APPLE)
    set(CMAKE_OSX_DEPLOYMENT_TARGET "10.11" CACHE STRING "Minimum OS X deployment version")
endif()

cmake_minimum_required(VERSION 3.2)
project(splashkit)

# Detect Windows and flag MSYS
if (WIN32 OR MSYS OR MINGW)
    SET(MSYS "true")

    string(COMPARE EQUAL "MINGW32" "$ENV{MSYSTEM}" MINGW32)
    string(COMPARE EQUAL "MINGW64" "$ENV{MSYSTEM}" MINGW64)

    if (${MINGW32})
        message("Using mingw32")
        set(PATH_SUFFIX "win32")
        set(MINGW_PATH_PART "mingw32")
    elseif (${MINGW64})
        message("Using mingw64")
        set(PATH_SUFFIX "win64")
        set(MINGW_PATH_PART "mingw64")
    else ( )
        message(SEND_ERROR "Failed to detect windows architecture")
        return ()
    endif()
elseif (APPLE)
    set(PATH_SUFFIX "macos")
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

# Setup OS specific flags
if (APPLE)
    # MAC OS PROJECT FLAGS
    set(LIB_FLAGS "-L${SK_LIB} \
                   -framework IOKit \
                   -framework ForceFeedback \
                   -framework CoreFoundation \
                   -framework Cocoa \
                   -framework Carbon \
                   -framework AudioUnit \
                   -framework AudioToolbox \
                   -framework CoreAudio \
                   -framework CoreVideo \
                   -lSDL2 \
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
                   -lpng16 \
                   -lvorbisfile \
                   -lmikmod \
                   -logg \
                   -lwebp \
                   -lsmpeg2 \
                   -lfreetype \
                   -lcurl \
                   -lncurses \
                   -ldl")

# WINDOWS PROJECT FLAGS
elseif(MSYS)
    list(REMOVE_ITEM C_SOURCE_FILES "source/civetweb.c")
    list(REMOVE_ITEM C_SOURCE_FILES "source/sqlite3.c")

    add_definitions(-DWINDOWS)

    set(LIB_FLAGS  "-L${SK_LIB}/${WIN_PATH_SUFFIX} \
                    -L/${MINGW_PATH_PART}/lib \
                    -L/usr/lib \
                    -lSDL2main \
                    -lSDL2_mixer \
                    -lSDL2_image \
                    -lSDL2_net \
                    -llibcivetweb \
                    -lSDL2 \
                    -lSDL2_ttf \
                    -llibcurl \
                    -llibSDL2_gfx-1-0-0 \
                    -llibpng16-16 \
                    -llibsqlite \
                    -lws2_32 \
                    -llibncursesw \
                    -static-libstdc++ -static-libgcc \
                    -Wl,-Bstatic -lstdc++ -lpthread -Wl,-Bdynamic"
                    )

    file(GLOB SK_LIB_DLL_FILES
        "${SK_LIB}/*.dll"
    )
# LINUX PROJECT FLAGS
else()
    find_package(PNG REQUIRED)
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
endif()

# FLAGS
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++14")

# MAC OS AND WINDOWS DIRECTORY INCLUDES
if (APPLE OR MSYS)
    include_directories("${SKM_ROOT}/source/include/extras/SDL/include")
    include_directories("${SKM_ROOT}/source/include/extras/SDL_gfx")
    include_directories("${SKM_ROOT}/source/include/extras/SDL_image")
    include_directories("${SKM_ROOT}/source/include/extras/SDL_mixer")
    include_directories("${SKM_ROOT}/source/include/extras/SDL_net")
    include_directories("${SKM_ROOT}/source/include/extras/SDL_ttf")
endif()

# MAC OS ONLY DIRECTORY INCLUDES
if (APPLE)
    include_directories("${SK_EXT}/SDL_image/external/libpng-1.6.2")
endif()

# WINDOWS ONLY DIRECTORY INCLUDES
if (MSYS)
    include_directories(/${MINGW_PATH_PART}/include)
    include_directories(/${MINGW_PATH_PART}/include/libpng16)
    include_directories("${SK_LIB}/win_inc")
endif()

# MACRO DEFINITIONS #
add_definitions(-DELPP_THREAD_SAFE)

#### END SETUP ####

#### SplashKitBackend STATIC LIBRARY ####


add_library(SplashKit SHARED ${SOURCE_FILES} ${C_SOURCE_FILES} ${OS_SOURCE_FILES} ${INCLUDE_FILES})
target_link_libraries(SplashKit ${LIB_FLAGS})

if (LINUX)
    target_link_libraries(SplashKit ${PNG_LIBRARY})
endif()

get_filename_component(SK_DEPLOY_ROOT ${SK_DEPLOY_ROOT} ABSOLUTE)

install(TARGETS SplashKit DESTINATION ${SK_DEPLOY_ROOT})

if (LINUX)
    INSTALL(CODE "execute_process( \
       COMMAND ${CMAKE_COMMAND} -E create_symlink \
       libSplashKit.so \
       ${SK_DEPLOY_ROOT}/splashkit.dll   \
       )"
    )
endif()

if ( MSYS )
  install(FILES ${SK_LIB_DLL_FILES} DESTINATION ${SK_DEPLOY_ROOT})
endif()
