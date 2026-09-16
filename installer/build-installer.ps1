# Bangun installer EvernightLanguage (Fase 6F).
#
# Alur:
#   1. Bangun biner compiler rilis (evernight.exe) dari workspace root
#   2. Salin berkas payload ke installer/aset/payload/
#   3. Bangun installer (crate terpisah, di luar workspace)
#   4. Hasilkan installer/EvernightLanguage-0.1.0-Setup.exe + checksum
#
# Pemakaian:
#   powershell -ExecutionPolicy Bypass -File installer/build-installer.ps1
#   powershell ... -File installer/build-installer.ps1 -LewatiCompiler

param(
    [switch]$LewatiCompiler,
    [string]$Versi = "0.1.0"
)

$ErrorActionPreference = "Stop"
$akar = Split-Path -Parent $PSScriptRoot
$inst = $PSScriptRoot

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
    # Fallback: lokasi target khusus yang dipakai sesi pengembangan.
    $alt = Join-Path $env:TEMP "opencode\eve-target\release\evernight.exe"
    if (Test-Path $alt) { $exeSumber = $alt }
    else { throw "evernight.exe tidak ditemukan. Jalankan tanpa -LewatiCompiler." }
}

# Cegah paket berisi biner BASI: bandingkan waktu ubah biner dengan sumber
# terbaru di crates/. Ini pernah menyebabkan installer membawa compiler lama
# yang belum punya subcommand `format`/`lint`/`pkg`.
$waktuBiner = (Get-Item $exeSumber).LastWriteTime
$sumberTerbaru = Get-ChildItem (Join-Path $akar "crates") -Recurse -Filter "*.rs" |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1
if ($sumberTerbaru -and $sumberTerbaru.LastWriteTime -gt $waktuBiner) {
    $pesan = "Biner compiler LEBIH TUA dari sumber " +
             "($($sumberTerbaru.Name), $($sumberTerbaru.LastWriteTime))."
    if ($LewatiCompiler) {
        throw "$pesan Jalankan build-installer.ps1 TANPA -LewatiCompiler."
    }
    Write-Host "  $pesan Membangun ulang..." -ForegroundColor Yellow
}
Write-Host "  Biner: $exeSumber  ($waktuBiner)"

# --- 2. Payload --------------------------------------------------------
Write-Host "`n[2/4] Menyiapkan payload..." -ForegroundColor Yellow
$payload = Join-Path $inst "aset\payload"
New-Item -ItemType Directory -Force -Path $payload | Out-Null

Copy-Item $exeSumber (Join-Path $payload "evernight.exe") -Force
Write-Host "  evernight.exe"

$vsix = Join-Path $akar "paket\evernight-0.1.0-windows-x64\extensions\evernight-language-$Versi.vsix"
if (Test-Path $vsix) {
    Copy-Item $vsix (Join-Path $payload "evernight-language-$Versi.vsix") -Force
    Write-Host "  ekstensi .vsix"
}

$panduan = Join-Path $akar "paket\evernight-0.1.0-windows-x64\docs\PANDUAN.txt"
if (Test-Path $panduan) {
    Copy-Item $panduan (Join-Path $payload "PANDUAN.txt") -Force
    Write-Host "  PANDUAN.txt"
}

Copy-Item (Join-Path $akar "LICENSE") (Join-Path $payload "LICENSE") -Force
Write-Host "  LICENSE"

# --- 3. Installer ------------------------------------------------------
Write-Host "`n[3/4] Membangun installer (release)..." -ForegroundColor Yellow

# Toolchain GNU butuh `dlltool` untuk beberapa crate; pastikan tersedia.
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

$ukuran = [Math]::Round((Get-Item $keluar).Length / 1MB, 2)
$hash = (Get-FileHash $keluar -Algorithm SHA256).Hash.ToLower()
"$hash  $namaKeluar" | Set-Content (Join-Path $dist "SHA256SUMS.txt") -Encoding ASCII

Write-Host "`nSelesai." -ForegroundColor Green
Write-Host "  Berkas : $keluar"
Write-Host "  Ukuran : $ukuran MB"
Write-Host "  SHA256 : $hash"
