@echo off
REM uninstall.cmd - Pembungkus pencopotan agar bisa diklik dua kali.

setlocal
cd /d "%~dp0"

echo.
echo  Pencopot EvernightLanguage
echo  --------------------------
echo.

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0uninstall.ps1" %*

set HASIL=%ERRORLEVEL%
echo.
if not "%HASIL%"=="0" (
    echo  Pencopotan GAGAL dengan kode %HASIL%.
) else (
    echo  Pencopotan selesai.
)
echo.
pause
endlocal
exit /b %HASIL%
