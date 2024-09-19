Import-Module .\variables.psm1

Set-Location -Path $rust_path

cargo build --release --target x86_64-pc-windows-msvc
Pop-Location
ClearDLLs

if (Test-Path -Path $release_file) {
    Move-Item -Path $release_file -Destination $godot_release_file
}

