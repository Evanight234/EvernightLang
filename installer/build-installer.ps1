# Bangun installer EvernightLanguage (Fase 6F).
#
# Alur:
#   1. Bangun biner compiler rilis (evernight.exe) dari workspace root
#   2. Salin berkas payload ke installer/aset/payload/
#   3. Bangun installer (crate terpisah, di luar workspace)
#   4. Hasilkan installer/EvernightLanguage-<versi>-Setup.exe + checksum

param(
    [switch]$LewatiCompiler,
    [string]$Versi
)

$ErrorActionPreference = "Stop"
$akar = Split-Path -Parent $PSScriptRoot
$inst = $PSScriptRoot

if (-not $Versi) {
    $vf = Join-Path $akar "version\version"
    if (Test-Path $vf) { $Versi = (Get-Content $vf).Trim() }
    else { $Versi = "0.2.0" }
}

Write-Host "== Bangun Installer EvernightLanguage v$Versi ==" -ForegroundColor Cyan

# --- 1. Biner compiler -------------------------------------------------
if (-not $LewatiCompiler) {
    Write-Host "`n[1/4] Membangun compiler (release)..." -ForegroundColor Yellow
    Push-Location $akar
    cargo build --release -p evernight_cli
    Pop-Location
    if ($LASTEXITCODE -ne 0) { throw "Build compiler gagal" }
}

$exeSumber = Join-Path $akar "target\release\evernight.exe"
if (-not (Test-Path $exeSumber)) {
    throw "evernight.exe tidak ditemukan di target\release. Jalankan tanpa -LewatiCompiler."
}

$waktuBiner = (Get-Item $exeSumber).LastWriteTime
Write-Host "  Biner: $exeSumber  ($waktuBiner)"

# --- 2. Payload --------------------------------------------------------
Write-Host "`n[2/4] Menyiapkan payload..." -ForegroundColor Yellow
$payload = Join-Path $inst "aset\payload"
New-Item -ItemType Directory -Force -Path $payload | Out-Null

Copy-Item $exeSumber (Join-Path $payload "evernight.exe") -Force
Write-Host "  evernight.exe"

$vsix = Join-Path $akar "editors\vscode\evernight-language-$Versi.vsix"
if (-not (Test-Path $vsix)) {
    $vsix = Join-Path $akar "paket\evernight-$Versi-windows-x64\extensions\evernight-language-$Versi.vsix"
}
if (-not (Test-Path $vsix)) {
    $vsix = Join-Path $akar "paket\evernight-0.1.0-windows-x64\extensions\evernight-language-0.1.0.vsix"
}
if (Test-Path $vsix) {
    Copy-Item $vsix (Join-Path $payload "evernight-language.vsix") -Force
    Write-Host "  ekstensi .vsix: $vsix"
}

$panduan = Join-Path $akar "paket\evernight-$Versi-windows-x64\docs\PANDUAN.txt"
if (-not (Test-Path $panduan)) {
    $panduan = Join-Path $akar "paket\evernight-0.1.0-windows-x64\docs\PANDUAN.txt"
}
if (Test-Path $panduan) {
    Copy-Item $panduan (Join-Path $payload "PANDUAN.txt") -Force
    Write-Host "  PANDUAN.txt"
}

Copy-Item (Join-Path $akar "LICENSE") (Join-Path $payload "LICENSE") -Force
Write-Host "  LICENSE"

# Salin kill & update scripts ke payload
$killCmd = Join-Path $akar "kill.cmd"
if (Test-Path $killCmd) { Copy-Item $killCmd (Join-Path $payload "kill.cmd") -Force }
$killPs1 = Join-Path $akar "kill.ps1"
if (Test-Path $killPs1) { Copy-Item $killPs1 (Join-Path $payload "kill.ps1") -Force }
$updateCmd = Join-Path $akar "update.cmd"
if (Test-Path $updateCmd) { Copy-Item $updateCmd (Join-Path $payload "update.cmd") -Force }
$updatePs1 = Join-Path $akar "update.ps1"
if (Test-Path $updatePs1) { Copy-Item $updatePs1 (Join-Path $payload "update.ps1") -Force }

# --- 3. Installer ------------------------------------------------------
Write-Host "`n[3/4] Membangun installer (release)..." -ForegroundColor Yellow

$kandidatDlltool = @(
    "C:\msys64\ucrt64\bin",
    "C:\msys64\mingw64\bin"
)
foreach ($d in $kandidatDlltool) {
    if (Test-Path (Join-Path $d "dlltool.exe")) {
        if ($env:PATH -notlike "*$d*") { $env:PATH = "$d;$env:PATH" }
        break
    }
}

Push-Location $inst
cargo build --release
Pop-Location
if ($LASTEXITCODE -ne 0) { throw "Build installer gagal" }

$exeInstaller = Join-Path $inst "target\release\evernight-installer.exe"
if (-not (Test-Path $exeInstaller)) { throw "evernight-installer.exe tidak dihasilkan" }

# --- 4. Hasil ----------------------------------------------------------
Write-Host "`n[4/4] Menyusun keluaran..." -ForegroundColor Yellow
$dist = Join-Path $inst "dist"
New-Item -ItemType Directory -Force -Path $dist | Out-Null

$namaKeluar = "EvernightLanguage-$Versi-Setup.exe"
$keluar = Join-Path $dist $namaKeluar
Copy-Item $exeInstaller $keluar -Force

# Salin juga sebagai Setup.exe di paket
$setupPaket = Join-Path $akar "paket\evernight-$Versi-windows-x64\setup"
New-Item -ItemType Directory -Force -Path $setupPaket | Out-Null
Copy-Item $exeInstaller (Join-Path $setupPaket "Setup.exe") -Force

$ukuran = [Math]::Round((Get-Item $keluar).Length / 1MB, 2)
$hash = (Get-FileHash $keluar -Algorithm SHA256).Hash.ToLower()
"$hash  $namaKeluar" | Set-Content (Join-Path $dist "SHA256SUMS.txt") -Encoding ASCII

Write-Host "`nSelesai." -ForegroundColor Green
Write-Host "  Berkas : $keluar"
Write-Host "  Paket  : $(Join-Path $setupPaket 'Setup.exe')"
Write-Host "  Ukuran : $ukuran MB"
Write-Host "  SHA256 : $hash"
