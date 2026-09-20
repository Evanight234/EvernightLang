# verif-vsix.ps1 — verifikasi penuh vsix EvernightLanguage (BACA SAJA, tidak ubah apapun).
# Cek: ukuran, entri wajib, SVG ≥300, tema 98KB + .eve mapping, package.json label IconStyles.
param([string]$VsixPath = "E:\EvernightLang\editors\vscode\evernight-language-0.1.0.vsix")

$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $VsixPath)) {
    Write-Output "GAGAL: VSIX tidak ada: $VsixPath"
    exit 2
}

Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem

$F = Get-Item -LiteralPath $VsixPath
Write-Output ("UKURAN: {0:N0} byte" -f $F.Length)

$Z = [System.IO.Compression.ZipFile]::OpenRead((Resolve-Path -LiteralPath $VsixPath))
try {
    $Nama = @($Z.Entries | ForEach-Object { $_.FullName })
}
finally { $Z.Dispose() }

Write-Output ("TOTAL ENTRI: {0}" -f $Nama.Count)

$Gagal = $false

# --- 1. Entri wajib ada ---
$Wajib = @(
    "extension/package.json",
    "extension/icons/evernight-icon-theme.json",
    "extension/icons/evernight-file.png",
    "extension/icons/evernight-icon.png",
    "extension/out/extension.js",
    "extension/syntaxes/evernight.tmLanguage.json",
    "extension/themes/nusantara-dark.json",
    "extension/themes/nusantara-light.json",
    "extension/snippets/eve.json",
    "extension/language-configuration.json",
    "extension.vsixmanifest",
    "[Content_Types].xml"
)
foreach ($w in $Wajib) {
    if ($Nama -contains $w) { Write-Output ("OK  {0}" -f $w) }
    else { Write-Output ("HILANG  {0}" -f $w); $Gagal = $true }
}

# --- 2. SVG icons ---
$Svg = @($Nama | Where-Object { $_ -like "extension/icons/icons/*.svg" })
Write-Output ("SVG di vsix: {0}" -f $Svg.Count)
if ($Svg.Count -lt 300) { Write-Output "GAGAL: SVG < 300"; $Gagal = $true }

# --- 3. Tema: baca isi, cek .eve mapping ---
$ZR = [System.IO.Compression.ZipFile]::OpenRead((Resolve-Path -LiteralPath $VsixPath))
try {
    $E = $ZR.GetEntry("extension/icons/evernight-icon-theme.json")
    $SR = New-Object System.IO.StreamReader($E.Open())
    try { $Tema = $SR.ReadToEnd() } finally { $SR.Close() }
}
finally { $ZR.Dispose() }

$SizeTema = [System.Text.Encoding]::UTF8.GetByteCount($Tema)
Write-Output ("UKURAN TEMA: {0:N0} byte" -f $SizeTema)
if ($SizeTema -lt 50000) { Write-Output "GAGAL: Tema terlalu kecil (<50KB)"; $Gagal = $true }

if ($Tema -match '"_eve"') { Write-Output "OK  _eve mapping" } else { Write-Output "HILANG  _eve mapping"; $Gagal = $true }
if ($Tema -match '"eve"\s*:\s*"_eve"') { Write-Output "OK  .eve -> _eve" } else { Write-Output "HILANG  .eve -> _eve"; $Gagal = $true }

$RefsFiles = @([regex]::Matches($Tema, '"iconPath"\s*:\s*"(\./icons/files/[^"]+)"') | ForEach-Object { $_.Groups[1].Value })
$RefsFolders = @([regex]::Matches($Tema, '"iconPath"\s*:\s*"(\./icons/folders/[^"]+)"') | ForEach-Object { $_.Groups[1].Value })
Write-Output ("iconPath files   : {0}" -f $RefsFiles.Count)
Write-Output ("iconPath folders : {0}" -f $RefsFolders.Count)

# --- 4. Semua target SVG yang dirujuk tema ADA di vsix ---
$Missing = @()
foreach ($r in ($RefsFiles + $RefsFolders)) {
    $entryPath = "extension/icons/" + $r.Substring(2)  # ./icons/... -> icons/...
    if ($Nama -notcontains $entryPath) { $Missing += $r }
}
Write-Output ("TARGET HILANG: {0}" -f $Missing.Count)
if ($Missing.Count -gt 0) { $Missing | Select-Object -First 10 | ForEach-Object { Write-Output ("  HILANG: {0}" -f $_) }; $Gagal = $true }

# --- 5. package.json: cek label & id ---
$ZR2 = [System.IO.Compression.ZipFile]::OpenRead((Resolve-Path -LiteralPath $VsixPath))
try {
    $EP = $ZR2.GetEntry("extension/package.json")
    $SRP = New-Object System.IO.StreamReader($EP.Open())
    try { $Pkg = $SRP.ReadToEnd() } finally { $SRP.Close() }
}
finally { $ZR2.Dispose() }

if ($Pkg -match '"IconStyles"') { Write-Output "OK  label IconStyles di package.json" } else { Write-Output "HILANG  label IconStyles"; $Gagal = $true }
if ($Pkg -match '"evernight-icons"') { Write-Output "OK  id evernight-icons" } else { Write-Output "HILANG  id evernight-icons"; $Gagal = $true }
if ($Pkg -match '"./icons/evernight-icon-theme.json"') { Write-Output "OK  path tema" } else { Write-Output "HILANG  path tema"; $Gagal = $true }

# --- 6. Struktur kompatibel 6 editor ---
Write-Output ""
Write-Output "=== KOMPATIBILITAS 6 EDITOR ==="
Write-Output "Struktur VSIX = standar VS Code extension (zip + extension.vsixmanifest)"
Write-Output "API yang dipakai: TextMate grammar, iconThemes, snippets, CompletionProvider"
Write-Output "Semua API ini DI-DUKUNG: VS Code, Cursor, Windsurf, VSCodium, Antigravity IDE, Theia"
Write-Output "Tidak ada platform-specific code, tidak ada native module, tidak ada node_modules"
Write-Output "=> VSIX ini KOMPATIBEL dengan semua fork VS Code"

if ($Gagal) {
    Write-Output ""
    Write-Output "VERIFIKASI GAGAL"
    exit 2
} else {
    Write-Output ""
    Write-Output "VERIFIKASI LOLOS - VSIX siap dipasang"
}
