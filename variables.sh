rust_src_dir="./rust-src"
debug_dir="./libs/macOS_64/debug"
debug_lib_path="./libs/macOS_64/debug/librust_src.dylib"
debug_output_lib="./rust-src/target/aarch64-apple-darwin/debug/librust_src.dylib"

function construct_catalogs_structure {
    if [ ! -d "$debug_dir" ]; then
        mkdir -p "$debug_dir"
    fi
}

function clear_libs {
    if [ -f "$debug_lib_path" ]; then
        rm "$debug_lib_path"
    fi
}