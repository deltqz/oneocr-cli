@echo off
setlocal EnableExtensions EnableDelayedExpansion

set "RUNDIR=%CD%"

:: Get Snipping Tool installation directory
for /f "usebackq delims=" %%I in (`
  powershell -NoProfile -Command "(Get-AppxPackage Microsoft.ScreenSketch).InstallLocation"
`) do (
  set "APPXDIR=%%I"
)

:: Check if directory was found
if not defined APPXDIR (
  echo ERROR: Failed to locate Microsoft.ScreenSketch
  exit /b 1
)

:: Copy files to target directory
set "SRC=%APPXDIR%\SnippingTool"

for %%F in (
  oneocr.dll
  oneocr.onemodel
  onnxruntime.dll
) do (
  if exist "%SRC%\%%F" (
    echo Copying %%F
    copy /Y "%SRC%\%%F" "%RUNDIR%" >nul
  ) else (
    echo WARNING: %%F not found
  )
)

echo.
echo Done.
exit /b 0
