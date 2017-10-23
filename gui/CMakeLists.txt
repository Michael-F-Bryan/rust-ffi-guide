project(rest-client)
set(CMAKE_CXX_STANDARD 14)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
find_package(Qt5Widgets)

add_executable(gui 
    main_window.cpp main_window.hpp main.cpp)
target_link_libraries(gui Qt5::Widgets)
add_dependencies(gui client)