source ./variables_linux_gnu.sh

clear_libs

cd "$rust_src_dir"
cargo build --release --target x86_64-unknown-linux-gnu
cd ..

if [ ! -f "$release_output_lib" ]; then
    echo "Build failed, check console for details."
    exit 1
fi

mv "$release_output_lib" "$release_lib_path"
