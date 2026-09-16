@echo off
REM install.cmd - Pembungkus agar pemasangan bisa dilakukan dengan klik dua kali.
REM Menggunakan -ExecutionPolicy Bypass supaya tidak terhalang kebijakan skrip PowerShell.

setlocal
cd /d "%~dp0"

echo.
echo  Pemasang EvernightLanguage
echo  --------------------------
echo  Memulai pemasangan (tanpa hak admin)...
echo.

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1" %*

set HASIL=%ERRORLEVEL%
echo.
if not "%HASIL%"=="0" (
    echo  Pemasangan GAGAL dengan kode %HASIL%.
) else (
    echo  Pemasangan selesai.
)
echo.
pause
endlocal
exit /b %HASIL%
