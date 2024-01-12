cmake_minimum_required (VERSION 3.8)

project ("StehendesLauflicht")

# Enable Hot Reload for MSVC compilers if supported.
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)
include(FetchContent)
message(STATUS "Downloading SFML...")
FetchContent_Declare(SFML
    GIT_REPOSITORY https://github.com/SFML/SFML.git
    GIT_TAG 7223a5d)
FetchContent_MakeAvailable(SFML)


add_executable (StehendesLauflicht "src/main.cpp" "src/Car.hpp" "src/Checkerboard.hpp")
target_link_libraries(StehendesLauflicht PRIVATE sfml-graphics)
add_dependencies(StehendesLauflicht sfml-graphics)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET StehendesLauflicht PROPERTY CXX_STANDARD 20)
endif()