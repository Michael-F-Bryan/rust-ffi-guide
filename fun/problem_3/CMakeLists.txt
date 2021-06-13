rustc_library(problem_3_lib 
              ${CMAKE_CURRENT_SOURCE_DIR}/home.rs 
              ${CMAKE_CURRENT_BINARY_DIR}/libhome.so)

add_executable(problem_3 ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
target_link_libraries(problem_3 ${CMAKE_CURRENT_BINARY_DIR}/libhome.so)
add_dependencies(problem_3 problem_3_lib)

install(TARGETS problem_3 DESTINATION bin)