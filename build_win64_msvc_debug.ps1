Import-Module .\variables.psm1

Set-Location -Path $rust_path

cargo build --verbose --target x86_64-pc-windows-msvc
Pop-Location

ClearDLLs

if (Test-Path -Path $debug_file) {
    Move-Item -Path $debug_file -Destination $godot_debug_file
}