$main_path = $PSScriptRoot
$rust_path = Join-Path -Path $main_path -ChildPath "rust-src"
$godot_release = Join-Path -Path $main_path -ChildPath "godot/libs/win64/release"
$godot_release_file = Join-Path -Path $godot_release -ChildPath "rust_src.dll"
$godot_debug = Join-Path -Path $main_path -ChildPath "godot/libs/win64/debug"
$godot_debug_file = Join-Path -Path $godot_debug -ChildPath "rust_src.dll"
$release_file = Join-Path -Path $rust_path -ChildPath "target/x86_64-pc-windows-msvc/release/rust_src.dll"
$debug_file  = Join-Path -Path $rust_path -ChildPath "target/x86_64-pc-windows-msvc/debug/rust_src.dll"

function ClearDLLs {
    Write-Host "Clearing Windows DLLs: release and debug."
    
    if (Test-Path -Path $godot_debug_file) {
        Remove-Item -Path $godot_debug_file
    }
    
    if (Test-Path -Path $godot_release_file) {
        Remove-Item -Path $godot_release_file
    }
    
}

Export-ModuleMember -Variable main_path, rust_path, godot_release, godot_release_file, godot_debug, godot_debug_file, release_file, debug_file -Function ClearDLLs
