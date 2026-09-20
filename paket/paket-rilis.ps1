<#
.SYNOPSIS
    Rakit paket rilis EvernightLanguage per versi.
.PARAMETER Versi
    Nomor versi (mis. 0.1.0). Wajib.
.EXAMPLE
    .\paket\paket-rilis.ps1 -Versi 0.2.0
#>
param(
    [Parameter(Mandatory = $true)]
    [string]$Versi
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
$NamaPaket = "evernight-$Versi-windows-x64"
$OutDir = Join-Path $PSScriptRoot $NamaPaket
$Payload = Join-Path $Root "installer/aset/payload"
$Vsix = Get-ChildItem (Join-Path $Root "editors/vscode") -Filter "*.vsix" | Select-Object -First 1

Write-Host "=== PAket RILIS $NamaPaket ===" -ForegroundColor Cyan

# 1. Build CLI release
Write-Host "[1/6] Build evernight.exe release..."
Push-Location $Root
$ErrorActionPreference_old = $ErrorActionPreference
$ErrorActionPreference = "Continue"
cargo build --release --bin evernight 2>&1 | Out-Null
$ErrorActionPreference = $ErrorActionPreference_old
Pop-Location
$ExeSrc = Join-Path $Root "target/release/evernight.exe"
if (-not (Test-Path $ExeSrc)) { throw "evernight.exe tidak ditemukan setelah build" }

# 2. Siapkan folder
Write-Host "[2/6] Siapkan folder paket..."
if (Test-Path $OutDir) { Remove-Item $OutDir -Recurse -Force }
$dirs = @("bin", "docs", "extensions", "assets", "setup")
foreach ($d in $dirs) { New-Item -ItemType Directory -Path (Join-Path $OutDir $d) -Force | Out-Null }

# 3. Salin file
Write-Host "[3/6] Salin file..."
Copy-Item $ExeSrc (Join-Path $OutDir "bin/evernight.exe")
# Payload: kill + update
Copy-Item (Join-Path $Payload "kill.cmd") (Join-Path $OutDir "bin/kill.cmd")
Copy-Item (Join-Path $Payload "kill.ps1") (Join-Path $OutDir "bin/kill.ps1")
Copy-Item (Join-Path $Payload "update.cmd") (Join-Path $OutDir "bin/update.cmd")
Copy-Item (Join-Path $Payload "update.ps1") (Join-Path $OutDir "bin/update.ps1")
# VSIX
if ($Vsix) { Copy-Item $Vsix.FullName (Join-Path $OutDir "extensions/$($Vsix.Name)") }
# Docs
Copy-Item (Join-Path $Payload "PANDUAN.txt") (Join-Path $OutDir "docs/PANDUAN.txt")
$CatatanSrc = Join-Path $Root "version/catatan-$Versi.txt"
if (Test-Path $CatatanSrc) { Copy-Item $CatatanSrc (Join-Path $OutDir "docs/catatan-$Versi.txt") }
# Assets
Copy-Item (Join-Path $Root "assets/icon.ico") (Join-Path $OutDir "assets/icon.ico") -ErrorAction SilentlyContinue
Copy-Item (Join-Path $Root "assets/icon_256.png") (Join-Path $OutDir "assets/icon_256.png") -ErrorAction SilentlyContinue
Copy-Item (Join-Path $Root "assets/logo.png") (Join-Path $OutDir "assets/logo.png") -ErrorAction SilentlyContinue
# LICENSE
Copy-Item (Join-Path $Payload "LICENSE") (Join-Path $OutDir "LICENSE")
# Setup installer (v2 only, renamed to Setup.exe)
$SetupSrc = Join-Path $Root "installer/dist/EvernightLanguage-$Versi-Setup-v2.exe"
if (Test-Path $SetupSrc) {
    Copy-Item $SetupSrc (Join-Path $OutDir "setup/Setup.exe")
} else {
    Write-Host "  [PERINGATAN] Setup v2 tidak ditemukan: $SetupSrc" -ForegroundColor Yellow
}

# 4. Generate file versi & BACA-AKU
Write-Host "[4/6] Generate versi.txt & BACA-AKU.txt..."
Set-Content -Path (Join-Path $OutDir "versi.txt") -Value $Versi -Encoding ASCII -NoNewline
$BacaAku = @"
EvernightLanguage $Versi - Paket Portabel
=========================================
Isi:
  bin/evernight.exe        Compiler + VM
  bin/kill.cmd             Uninstall (CMD)
  bin/kill.ps1             Uninstall (PowerShell)
  bin/update.cmd           Update (CMD)
  bin/update.ps1           Update (PowerShell)
  setup/Setup.exe          Installer GUI (wizard)
  extensions/              Ekstensi VS Code (.vsix)
  docs/PANDUAN.txt         Panduan penggunaan
  docs/catatan-$Versi.txt  Catatan rilis
  assets/                  Ikon & logo
  LICENSE                  Lisensi MIT

Mulai (pilih salah satu):
  [Cepat]  Jalankan install.cmd (atau install.ps1 di PowerShell)
  [Wizard] Klik ganda setup/Setup.exe, ikuti wizard
  Lalu: buka terminal baru, ketik: evernight --versi

Uninstall:
  kill evernight system (CMD) atau uninstall.ps1

Update:
  update evernight system
"@
Set-Content -Path (Join-Path $OutDir "BACA-AKU.txt") -Value $BacaAku -Encoding UTF8

# 5. Salin script install/uninstall (dari paket sumber jika ada)
Write-Host "[5/6] Salin install/uninstall script..."
$SrcInstall = Join-Path $Root "paket/evernight-0.1.0-windows-x64"
if (Test-Path (Join-Path $SrcInstall "install.ps1")) {
    Copy-Item (Join-Path $SrcInstall "install.ps1") (Join-Path $OutDir "install.ps1")
    Copy-Item (Join-Path $SrcInstall "install.cmd") (Join-Path $OutDir "install.cmd")
    Copy-Item (Join-Path $SrcInstall "uninstall.ps1") (Join-Path $OutDir "uninstall.ps1")
    Copy-Item (Join-Path $SrcInstall "uninstall.cmd") (Join-Path $OutDir "uninstall.cmd")
}

# 6. ZIP + SHA256
Write-Host "[6/6] Buat ZIP & SHA256..."
$ZipPath = Join-Path $PSScriptRoot "$NamaPaket.zip"
if (Test-Path $ZipPath) { Remove-Item $ZipPath -Force }
Compress-Archive -Path $OutDir -DestinationPath $ZipPath -Force

$ShaExe = (Get-FileHash (Join-Path $OutDir "bin/evernight.exe") -Algorithm SHA256).Hash.ToLower()
$ShaZip = (Get-FileHash $ZipPath -Algorithm SHA256).Hash.ToLower()
$ShaSetup = if (Test-Path (Join-Path $OutDir "setup/Setup.exe")) {
    (Get-FileHash (Join-Path $OutDir "setup/Setup.exe") -Algorithm SHA256).Hash.ToLower()
} else { "n/a" }
$ShaFile = Join-Path $PSScriptRoot "SHA256SUMS.txt"
@"
$ShaExe  $NamaPaket/bin/evernight.exe
$ShaSetup  $NamaPaket/setup/Setup.exe
$ShaZip  $NamaPaket.zip
"@ | Set-Content -Path $ShaFile -Encoding ASCII

Write-Host ""
Write-Host "SELESAI." -ForegroundColor Green
Write-Host "  Folder : $OutDir"
Write-Host "  ZIP    : $ZipPath"
Write-Host "  SHA256 : $ShaFile"
Write-Host "  exe    : $ShaExe"
Write-Host "  setup  : $ShaSetup"
