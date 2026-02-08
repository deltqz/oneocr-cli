@echo off
setlocal EnableExtensions EnableDelayedExpansion

set "RUNDIR=%~dp0"

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

:: Add OneOCR-CLI to PATH
echo Adding %RUNDIR% to PATH
powershell -NoP -C "$p=[Environment]::GetEnvironmentVariable('Path','User');$d='%~dp0'.TrimEnd('\');if($p -notlike '*'+$d+'*'){[Environment]::SetEnvironmentVariable('Path',\"$p;$d\",'User')}"

echo.
echo Done!
pause
exit /b 0
