rustc_library(problem_1_lib 
              ${CMAKE_CURRENT_SOURCE_DIR}/adder.rs 
              ${CMAKE_CURRENT_BINARY_DIR}/libadder.so)

option(WITH_PROBLEM_1 "Try to compile problem 1" OFF)

if(WITH_PROBLEM_1)
    add_executable(problem_1 ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp)
    target_link_libraries(problem_1 ${CMAKE_CURRENT_BINARY_DIR}/libadder.so)
    add_dependencies(problem_1 problem_1_lib)

    install(TARGETS problem_1 DESTINATION bin)
endif(WITH_PROBLEM_1)

