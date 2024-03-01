$version = $args[0].replace("v", "")

$files = @(
    "./package.json",
    "./src-tauri/tauri.conf.json",
    "./src-tauri/Cargo.toml"
)

foreach ($file in $files) {
    ((Get-Content -path $file -Raw) -replace '0.0.0', $version) | Set-Content -Path $file
}