# OneOCR-CLI
Simple CLI tool to extract the text from an image using OneOCR, the OCR engine used by W11's Snipping Tool. Based on b1tg work, who found out how it works via reverse engineering. Main differences from the original project: rewritten in Rust, no need for OpenCV at all, plain output (no numebered lines). Only tested on W11.

## Usage example
In the terminal:
```cmd
oneocr image_001.png >text.txt
```
## Installation:
