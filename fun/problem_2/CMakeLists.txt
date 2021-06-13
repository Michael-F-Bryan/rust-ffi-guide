rustc_library(problem_2_lib 
              ${CMAKE_CURRENT_SOURCE_DIR}/foo.rs 
              ${CMAKE_CURRENT_BINARY_DIR}/libfoo.so)

add_executable(problem_2 ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
target_link_libraries(problem_2 ${CMAKE_CURRENT_BINARY_DIR}/libfoo.so)
add_dependencies(problem_2 problem_2_lib)

install(TARGETS problem_2 DESTINATION bin)

