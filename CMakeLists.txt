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
        COMMAND rustc --crate-type cdylib ${source_file} -o ${output_library}
        DEPENDS ${source_file})
    add_custom_target(${target_name} DEPENDS ${output_library})
endfunction(rustc_library)


enable_testing()
add_subdirectory(client)
add_subdirectory(injector-plugin)
add_subdirectory(gui)
add_subdirectory(book/fun)

