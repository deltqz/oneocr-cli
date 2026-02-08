# OneOCR-CLI
Simple CLI tool to extract the text from an image using OneOCR, the OCR engine used by W11's Snipping Tool. Based on b1tg work, who found out how it works via reverse engineering. Main differences from the original project: rewritten in Rust, no need for OpenCV at all, plain output (no numbered lines).

## Usage example
In the terminal:
```cmd
oneocr image_001.png >text.txt
```

## Prerequisites:
- Windows 11 (x64)
- [Snipping Tool](https://apps.microsoft.com/detail/9mz95kl8mr0l)
- PowerShell

## Installation:
1 - Get the latest build from the releases section.<br>
2 - Extract the `OneOCR` folder to any directory.<br>
3 - Execute the included `install.bat` file, then delete it if you want. Done!

This last step is neccesary to copy `oneocr.dll`, `oneocr.onemodel` and `onnxruntime.dll` from the Snipping Tool folder. This also adds `oneocr` to PATH, so you can run it from any folder via CLI or batching. If you copy the required files and add the OneOCR folder to PATH manually, you don't have to run the bat at all. PowerShell is only needed by the bat file to perform those two tasks, it's not a OneOCR-CLI dependency.

I can't include the required files in the release because of potential licensing issues, since it's privative software that you already have on your PC anyway. For me, the files are in this location, but it may be different for you.
```cmd
C:\Program Files\WindowsApps\Microsoft.ScreenSketch_11.2510.31.0_x64__8wekyb3d8bbwe\SnippingTool
```
