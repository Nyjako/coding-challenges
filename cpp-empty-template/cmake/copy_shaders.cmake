set(SHADERS_DIR "${CMAKE_SOURCE_DIR}/shaders")
set(OUTPUT_DIR "${CMAKE_BINARY_DIR}/shaders")

add_custom_command(
    TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    ${SHADERS_DIR} ${OUTPUT_DIR}
    COMMENT "Copying shaders directory to output."
)