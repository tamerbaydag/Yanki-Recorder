@echo off
setlocal

echo ============================================
echo   Yanki-Recorder - Windows Build Script
echo ============================================
echo.

REM Check if Rust is installed
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo [HATA] Rust bulunamadi.
    echo rustup.rs adresinden Rust'u kurduktan sonra bu dosyayi tekrar calistirin.
    pause
    exit /b 1
)

echo [OK] Rust bulundu:
rustc --version
cargo --version
echo.

REM Check if we're in the project directory
if not exist "Cargo.toml" (
    echo [HATA] Cargo.toml bulunamadi.
    echo Bu dosyayi Yanki-Recorder projesinin ana dizininden calistirin.
    pause
    exit /b 1
)

echo [OK] Proje dizini dogru.
echo.

REM Ask for build mode
echo Hangi modda derleyelim?
echo   1) Release (son kullanici icin, optimize)
echo   2) Debug (test icin, hizli derleme)
echo.
set /p MODE="Secim (1 veya 2, varsayilan 1): "

if "%MODE%"=="2" (
    set BUILD_FLAG=""
    set OUT_DIR=target\debug
    echo [INFO] Debug modunda derleniyor...
) else (
    set BUILD_FLAG=--release
    set OUT_DIR=target\release
    echo [INFO] Release modunda derleniyor...
)
echo.

REM Build
echo Derleme basliyor, bu surede kilitlanmayin...
echo.
cargo build %BUILD_FLAG%
if %errorlevel% neq 0 (
    echo.
    echo [HATA] Derleme basarisiz oldu.
    echo Yukaridaki hatalari kontrol edin.
    pause
    exit /b 1
)

echo.
echo ============================================
echo   Derleme basariyla tamamlandi!
echo ============================================
echo.
echo Cikti dosyasi: %OUT_DIR%\yanki-recorder.exe
echo.

REM Check if exe exists
if exist "%OUT_DIR%\yanki-recorder.exe" (
    echo [OK] yanki-recorder.exe bulundu.
    echo.
    set /p RUN="Uygulamayi simdi calistiralim mi? (E/H): "
    if /i "%RUN%"=="E" (
        start "" "%OUT_DIR%\yanki-recorder.exe"
        echo Uygulama baslatildi.
    )
) else (
    echo [UYARI] Beklenen exe dosyasi bulunamadi.
)

echo.
pause
endlocal
