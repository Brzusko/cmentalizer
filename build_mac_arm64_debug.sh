source ./variables.sh

construct_catalogs_structure
clear_libs

cd "$rust_src_dir"
cargo build --verbose --target aarch64-apple-darwin
cd ..

if [ ! -f "$debug_output_lib" ]; then
    echo "Build failed, check console for details."
    exit 1
fi

mv "$debug_output_lib" "$debug_lib_path"