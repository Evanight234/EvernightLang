# kill.ps1 - Wrapper untuk `kill evernight system` di PowerShell.
#
# Masalah: `kill` di PowerShell adalah alias untuk Stop-Process.
# Solusi: function ini mendefinisikan `kill` yang mendeteksi argumen
# "evernight" dan mengalihkan ke uninstaller.
#
# Dipanggil oleh installer saat pemasangan (ditambahkan ke $PROFILE).
# Dihapus oleh uninstaller saat pencopotan.

function kill {
    param(
        [Parameter(Position=0)]
        $Name,
        [Parameter(Position=1, ValueFromRemainingArguments)]
        $Remaining
    )

    if ($Name -eq 'evernight') {
        # Cari uninstall.exe dari registri.
        $regPath = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage'
        $lokasi = (Get-ItemProperty -Path $regPath -ErrorAction SilentlyContinue).InstallLocation
        if (-not $lokasi) {
            $lokasi = Join-Path $env:LOCALAPPDATA 'Programs\Evernight'
        }
        $uninstall = Join-Path $lokasi 'uninstall.exe'

        if (Test-Path $uninstall) {
            & $uninstall --uninstall @Remaining
        } else {
            Write-Host ''
            Write-Host '  EvernightLanguage tidak terpasang atau uninstall.exe tidak ditemukan.'
            Write-Host ''
        }
    } else {
        # Fallback ke Stop-Process bawaan PowerShell.
        Stop-Process -Name $Name @Remaining
    }
}
