cmake_minimum_required(VERSION 3.7)
project(rest-client)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_RELEASE_FLAG "" CACHE INTERNAL "")
    set(TARGET_DIR "debug" CACHE INTERNAL "")
else ()
    set(CARGO_RELEASE_FLAG --release CACHE INTERNAL "")
    set(TARGET_DIR "release" CACHE INTERNAL "")
endif ()


# Compile a the provided source file to be a shared object with a custom 
# target name.
function(rustc_library target_name source_file output_library)
    add_custom_command(OUTPUT ${output_library}
        COMMENT "Compiling ${target_name}"
        COMMAND rustc --crate-type cdylib ${source_file} -o ${output_library}
        DEPENDS ${source_file})
    add_custom_target(${target_name} DEPENDS ${output_library})
endfunction(rustc_library)


# Compile a Rust crate and copy the *.so into the current binary dir. This 
# also sets the LOCATION of the provided target to be the generated binary.
#
# A test target named ${target_name}_test is also generated.
function(cargo_library target_name project_dir)
    set(output_library ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/lib${target_name}.so)
    file(GLOB sources ${project_dir}/src/**/*.rs)

    add_custom_command(OUTPUT ${output_library}
        COMMENT "Compiling ${target_name}"
        COMMAND env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo build ${CARGO_RELEASE_FLAG}
        COMMAND cp ${output_library} ${CMAKE_CURRENT_BINARY_DIR}
        WORKING_DIRECTORY ${project_dir}
        DEPENDS ${sources})

    add_custom_target(${target_name} ALL DEPENDS ${output_library})
    set_target_properties(${target_name} PROPERTIES LOCATION ${output_library})

    add_test(NAME ${target_name}_test 
        COMMAND env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo test ${CARGO_RELEASE_FLAG}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
endfunction(cargo_library)

enable_testing()
add_subdirectory(client)
add_subdirectory(injector-plugin)
add_subdirectory(gui)
add_subdirectory(book/fun)

