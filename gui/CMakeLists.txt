set(CMAKE_CXX_STANDARD 14)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
find_package(Qt5Widgets)

set(CLIENT_BUILD_DIR ${CMAKE_BINARY_DIR}/client)
include_directories(${CLIENT_BUILD_DIR})

set(SOURCE main_window.cpp main_window.hpp wrappers.cpp wrappers.hpp main.cpp)

add_executable(gui ${SOURCE})

get_target_property(CLIENT_DIR client LOCATION)
target_link_libraries(gui Qt5::Widgets)
target_link_libraries(gui ${CLIENT_DIR}/libclient.so)
add_dependencies(gui client)
