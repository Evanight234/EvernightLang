# build-vsix.ps1 — bangun .vsix EvernightLanguage tanpa vsce/npx.
# Strategi: patch zip vsix yang sudah ada (Update mode) + verifikasi isi.
param([string]$Versi = "0.1.0")

$ErrorActionPreference = "Stop"
$Dalam = Split-Path -Parent $MyInvocation.MyCommand.Path
$Out = Join-Path $Dalam "evernight-language-$Versi.vsix"

if (-not (Test-Path -LiteralPath $Out)) { throw "Vsix dasar tidak ada: $Out" }

Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem

# Entri yang harus selalu dirombak (asli/modifikasi dari disk).
$Disk = @{
    "extension/icons/evernight-file.png"         = Join-Path $Dalam "icons\evernight-file.png"
    "extension/icons/evernight-icon-theme.json"  = Join-Path $Dalam "icons\evernight-icon-theme.json"
    "extension/icons/evernight-icon.png"         = Join-Path $Dalam "icons\evernight-icon.png"
}

function Tulis-Entri($zip, $tujuan, $asal) {
    $e = $zip.CreateEntry($tujuan, [System.IO.Compression.CompressionLevel]::Optimal)
    $s = $e.Open()
    try { $b = [IO.File]::ReadAllBytes((Resolve-Path $asal)); $s.Write($b, 0, $b.Length) }
    finally { $s.Close() }
}

$fs = [IO.File]::Open($Out, [IO.FileMode]::Open, [IO.FileAccess]::ReadWrite)
$zip = New-Object System.IO.Compression.ZipArchive($fs, [System.IO.Compression.ZipArchiveMode]::Update)
try {
    foreach ($k in $Disk.Keys) {
        $lama = $zip.Entries | Where-Object { $_.FullName -eq $k }
        if ($lama) { $lama | ForEach-Object { $_.Delete() } }
        Tulis-Entri $zip $k $Disk[$k]
    }
}
finally {
    $zip.Dispose(); $fs.Close()
}

Write-Output "VSIX OK: $Out"
Write-Output ("Ukuran: {0:N0} byte" -f (Get-Item $Out).Length)

# Verifikasi isi.
$Ver = @(
    "extension/icons/evernight-file.png",
    "extension/icons/evernight-icon-theme.json",
    "extension/icons/evernight-icon.png",
    "extension/package.json",
    "extension.vsixmanifest",
    "[Content_Types].xml"
)
$z = [System.IO.Compression.ZipFile]::OpenRead((Resolve-Path $Out))
$Nama = $z.Entries | ForEach-Object { $_.FullName }
$z.Dispose()
$Gagal = $false
foreach ($v in $Ver) {
    if ($Nama -contains $v) { Write-Output ("OK  {0}" -f $v) } else { Write-Output ("MISSING  {0}" -f $v); $Gagal = $true }
}
if ($Gagal) { exit 2 }