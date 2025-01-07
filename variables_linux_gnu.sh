rust_src_dir="./rust-src"
debug_dir="./libs/linux_x86_64/debug"
debug_lib_path="./libs/linux_x86_64/debug/librust_src.so"
debug_output_lib="./rust-src/target/x86_64-unknown-linux-gnu/debug/librust_src.so"
release_output_lib="./rust-src/target/x86_64-unknown-linux-gnu/release/librust_src.so"
release_lib_path="./godot/libs/linux_x86_64/release/librust_src.so"

function construct_catalogs_structure {
    if [ ! -d "$debug_dir" ]; then
        mkdir -p "$debug_dir"
    fi
}

function clear_libs {
    if [ -f "$debug_lib_path" ]; then
        rm "$debug_lib_path"
    fi

    if [ -f "$release_lib_path" ]; then
        rm "$release_lib_path";
    fi
}
