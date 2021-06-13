rustc_library(problem_4_lib 
              ${CMAKE_CURRENT_SOURCE_DIR}/logging.rs 
              ${CMAKE_CURRENT_BINARY_DIR}/liblogging.so)

add_executable(problem_4 ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
target_link_libraries(problem_4 ${CMAKE_CURRENT_BINARY_DIR}/liblogging.so)
add_dependencies(problem_4 problem_4_lib)

install(TARGETS problem_4 DESTINATION bin)