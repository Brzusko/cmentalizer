source ./variables_linux_gnu.sh

construct_catalogs_structure
clear_libs

cd "$rust_src_dir"
cargo build --verbose --target x86_64-unknown-linux-gnu
cd ..

if [ ! -f "$debug_output_lib" ]; then
    echo "Build failed, check console for details."
    exit 1
fi

mv "$debug_output_lib" "$debug_lib_path"
