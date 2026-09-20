# build-vsix.ps1 - bangun vsix EvernightLanguage PENUH (semua ikon SVG + tema gabungan).
# Strategi: buka vsix dasar vsce (zip Update mode), HAPUS entri icons/ lama + package.json,
# lalu TULIS dari disk: package.json berlabel IconStyles + SEMUA berkas folder icons\ (rekursif).
param([string]$Versi = "0.1.0")

$ErrorActionPreference = "Stop"
$Dalam        = Split-Path -Parent $MyInvocation.MyCommand.Path
$VsixDasar    = Join-Path $Dalam "evernight-language-$Versi.vsix"
$Out          = $VsixDasar

if (-not (Test-Path -LiteralPath $VsixDasar)) { throw "Vsix dasar tidak ada: $VsixDasar" }

Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem
$ZipOpsi = [System.IO.Compression.CompressionLevel]::Optimal

function Tulis-Entri([System.IO.Compression.ZipArchive]$Zip, [string]$Nama, [string]$AsalDisk) {
    $E = $Zip.CreateEntry($Nama, $ZipOpsi)
    $S = $E.Open()
    try {
        $B = [IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $AsalDisk))
        $S.Write($B, 0, $B.Length)
    }
    finally { $S.Close() }
}

# --- Folder ikon di disk (tema gabungan + 353 SVG Symbols). ---
$FolderIkon  = Join-Path $Dalam "icons"
$BerkasIkon  = @(Get-ChildItem -LiteralPath $FolderIkon -Recurse -File)
Write-Output ("berkas ikon di disk: {0}" -f $BerkasIkon.Count)
$JumlahSVG   = @($BerkasIkon | Where-Object { $_.Extension -eq ".svg" }).Count
Write-Output ("SVG di disk         : {0}" -f $JumlahSVG)
if ($JumlahSVG -lt 300) { throw "SVG < 300 - cek folder icons\icons\files & icons\folders" }

# --- Buka zip vsix (Update). ---
$FS = [IO.File]::Open($VsixDasar, [IO.FileMode]::Open, [IO.FileAccess]::ReadWrite)
$Zip = New-Object System.IO.Compression.ZipArchive($FS, [System.IO.Compression.ZipArchiveMode]::Update)
try {
    $Hapus = @($Zip.Entries | Where-Object {
        $_.FullName -like "extension/icons/*" -or $_.FullName -eq "extension/package.json"
    })
    Write-Output ("entri icons/package.json lama dihapus: {0}" -f $Hapus.Count)
    foreach ($H in $Hapus) { $H.Delete() }

    Tulis-Entri $Zip "extension/package.json" (Join-Path $Dalam "package.json")

    foreach ($F in $BerkasIkon) {
        $Rel = $F.FullName.Substring($FolderIkon.Length + 1) -replace "\\", "/"
        Tulis-Entri $Zip ("extension/icons/" + $Rel) $F.FullName
    }
}
finally {
    $Zip.Dispose()
    $FS.Close()
}

Write-Output ("SELESAI: {0}" -f $Out)
Write-Output ("Ukuran : {0:N0} byte" -f (Get-Item -LiteralPath $Out).Length)

# --- Verifikasi isi (read-only, bounded). ---
$Verifikasi = @(
    "extension/package.json",
    "extension/icons/evernight-icon-theme.json",
    "extension/icons/evernight-file.png",
    "extension/icons/evernight-icon.png",
    "extension/icons/icons/files/html.svg",
    "extension/icons/icons/folders/folder.svg",
    "extension/out/extension.js",
    "extension.vsixmanifest",
    "[Content_Types].xml"
)
$ZR = [System.IO.Compression.ZipFile]::OpenRead((Resolve-Path -LiteralPath $Out))
try {
    $Nama = @($ZR.Entries | ForEach-Object { $_.FullName })
}
finally { $ZR.Dispose() }

$Gagal = $false
foreach ($V in $Verifikasi) {
    if ($Nama -contains $V) { Write-Output ("OK  {0}" -f $V) }
    else { Write-Output ("MISSING  {0}" -f $V); $Gagal = $true }
}
$JumlahSVGZX = @($Nama | Where-Object { $_ -like "extension/icons/icons/*.svg" }).Count
Write-Output ("SVG di vsix  : {0}" -f $JumlahSVGZX)
if ($JumlahSVGZX -lt 300) {
    Write-Output "PERINGATAN: SVG < 300 - cek folder icons\icons\files & icons\folders"
    $Gagal = $true
}
if ($Gagal) { exit 2 }
exit 0
