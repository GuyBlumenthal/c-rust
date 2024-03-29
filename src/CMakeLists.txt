function(generate_c_bindings TARGET)
  add_custom_command(
    OUTPUT ${TARGET}-bindings
    COMMAND cbindgen ARGS --config ${CBINDGEN_TOML} --output ${CMAKE_CURRENT_BINARY_DIR}/inc/lib.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Generating C bindings for rust code for target ${TARGET}"
  )

  add_custom_target(${TARGET}-bindings-target ALL DEPENDS ${TARGET}-bindings)
endfunction()

function(c_rust_llvm TARGET SRC_FILES)
    generate_c_bindings(${TARGET})

    cargo_build(NAME ${TARGET}-lib)

    add_executable(${TARGET} ${SRC_FILES} ${CMAKE_CURRENT_SOURCE_DIR}/../seahorn.c)

    target_compile_options(${TARGET} PUBLIC 
        "-flto"
        "-O2"
        "--target=x86_64-unknown-linux-gnu" # TODO: Set this dynamically
    )

    target_link_libraries(${TARGET} ${TARGET}-lib )

    # TODO: Find a way to include seahorn.h more cleanly
    target_include_directories(${TARGET} PUBLIC 
      ${CMAKE_CURRENT_SOURCE_DIR}/..
      ${CMAKE_CURRENT_BINARY_DIR}/
    ) 

    target_link_options(${TARGET} PUBLIC
        "-flto"
        "-fuse-ld=lld-14"
        "-Wl,--plugin-opt=-lto-embed-bitcode=post-merge-pre-opt"
        "--target=x86_64-unknown-linux-gnu"
        "-O2"
    )

    SET(BC_FILE ${TARGET}_llvm.bc)

    add_custom_command(
        OUTPUT ${TARGET}-bc
        COMMAND llvm-objcopy-14 ARGS ${CMAKE_CURRENT_BINARY_DIR}/${TARGET} --dump-section .llvmbc=${BC_FILE}
        DEPENDS ${TARGET}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Copying llvm bitcode sections from executable for ${TARGET}"
    )

    add_custom_command(
        OUTPUT ${TARGET}-llvm
        COMMAND llvm-dis-14 ARGS ${CMAKE_CURRENT_BINARY_DIR}/${BC_FILE}
        DEPENDS ${TARGET}-bc
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Outputting disassembled program post merge pre opt llvm code for ${TARGET}"
    )

    add_custom_target(${TARGET}.ir ALL DEPENDS ${TARGET}-llvm)
    
    set_target_properties(${TARGET}.ir PROPERTIES
        LLVMIR_DIR ${CMAKE_CURRENT_BINARY_DIR}
        LLVMIR_FILES ${BC_FILE}
    )
endfunction()

function(sea_get_file_name VAR LLVMIR_TARGET)
  get_property(DIR TARGET ${LLVMIR_TARGET} PROPERTY LLVMIR_DIR)
  get_property(FILES TARGET ${LLVMIR_TARGET} PROPERTY LLVMIR_FILES)
  set(${VAR} "${DIR}/${FILES}" PARENT_SCOPE)
endfunction()

# Unit test for testing unsat
function(sea_add_unsat_test TARGET)
  sea_get_file_name(BC ${TARGET}.ir)
  add_test(NAME "${TARGET}_unsat_test" COMMAND ${VERIFY_CMD} ${VERIFY_FLAGS} --expect=unsat ${BC})
endfunction()

# Unit test for testing sat
function(sea_add_sat_test TARGET)
  sea_get_file_name(BC ${TARGET}.ir)
  add_test(NAME "${TARGET}_sat_test" COMMAND ${VERIFY_CMD} ${VERIFY_FLAGS} --expect=sat ${BC})
endfunction()

# Add "Jobs"
add_subdirectory(rust-jobs/add)
add_subdirectory(rust-jobs/vec)
add_subdirectory(rust-jobs/enum-as-param)
add_subdirectory(rust-jobs/option)
add_subdirectory(rust-jobs/pointer)

