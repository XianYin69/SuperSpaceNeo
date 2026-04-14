#Source Diractory of project file
$RootDir = Get-Location
$RustDir = "$RootDir\..\..\src\rust\rust_fileFunc"
$BuildDir = "$RootDir\..\..\src"

Write-Host "--- Step 1: Building Rust Static Library ---" -ForegroundColor Cyan
Push-Location $RustDir
# 强制 Rust 使用 MSVC 目标
cargo build --release --target x86_64-pc-windows-msvc
if ($LASTEXITCODE -ne 0) { Write-Error "Rust Build Failed"; exit }
Pop-Location

Write-Host "--- Step 2: Preparing Build Directory ---" -ForegroundColor Cyan
if (Test-Path $BuildDir) { Remove-Item -Recurse -Force $BuildDir }
New-Item -ItemType Directory -Path $BuildDir

Write-Host "--- Step 3: Running CMake ---" -ForegroundColor Cyan
cd $BuildDir
# 显式指定使用 Visual Studio 生成器以匹配 Rust 的 .lib
cmake -G "Visual Studio 17 2022" -A x64 ..
if ($LASTEXITCODE -ne 0) { Write-Error "CMake Configuration Failed"; exit }

Write-Host "--- Step 4: Compiling Project ---" -ForegroundColor Cyan
cmake --build . --config Release
if ($LASTEXITCODE -ne 0) { Write-Error "Compilation Failed"; exit }

Write-Host "--- Build Success! ---" -ForegroundColor Green