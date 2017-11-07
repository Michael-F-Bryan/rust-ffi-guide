if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libinjector_plugin.so")

add_custom_target(injector_plugin ALL
    COMMENT "Compiling injector_plugin module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${CLIENT_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(injector_plugin PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})
