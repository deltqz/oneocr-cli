use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use anyhow::{Context, Result};
use libloading::{Library, Symbol};

#[repr(C)]
#[derive(Debug)]
struct Img {
    t: i32,        // image type (3 = BGRA)
    col: i32,      // width
    row: i32,      // height
    unk: i32,      // unused / padding
    step: i64,     // bytes per row
    data_ptr: i64, // pointer to pixel buffer
}

/* Function pointer types (C ABI) */

type CreateOcrInitOptions = unsafe extern "C" fn(*mut i64) -> i64;
type OcrInitOptionsSetUseModelDelayLoad = unsafe extern "C" fn(i64, u8) -> i64;
type CreateOcrPipeline = unsafe extern "C" fn(i64, i64, i64, *mut i64) -> i64;
type CreateOcrProcessOptions = unsafe extern "C" fn(*mut i64) -> i64;
type OcrProcessOptionsSetMaxRecognitionLineCount = unsafe extern "C" fn(i64, i64) -> i64;
type RunOcrPipeline = unsafe extern "C" fn(i64, *const Img, i64, *mut i64) -> i64;
type GetOcrLineCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
type GetOcrLine = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
type GetOcrLineContent = unsafe extern "C" fn(i64, *mut i64) -> i64;

/* Load images and convert them to BGRA format */

fn load_image_bgra(path: &str) -> Result<(Img, Vec<u8>)> {
    let img = image::open(path)
        .with_context(|| format!("Failed to open image: {}", path))?;

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let mut bgra = Vec::with_capacity((width * height * 4) as usize);

    for px in rgba.chunks_exact(4) {
        bgra.push(px[2]); // B
        bgra.push(px[1]); // G
        bgra.push(px[0]); // R
        bgra.push(px[3]); // A
    }

    let step = (width * 4) as i64;

    let img_struct = Img {
        t: 3,
        col: width as i32,
        row: height as i32,
        unk: 0,
        step,
        data_ptr: bgra.as_ptr() as i64,
    };

    Ok((img_struct, bgra))
}

/* OCR logic */

unsafe fn run_ocr(img: &Img) -> Result<()> { unsafe {
    let lib = Library::new("oneocr.dll")
        .context("Failed to load oneocr.dll")?;

    macro_rules! sym {
        ($name:literal, $ty:ty) => {
            lib.get::<$ty>(concat!($name, "\0").as_bytes())
                .with_context(|| format!("Missing symbol: {}", $name))?
        };
    }

    let create_init: Symbol<CreateOcrInitOptions> =
        sym!("CreateOcrInitOptions", CreateOcrInitOptions);
    let set_delay: Symbol<OcrInitOptionsSetUseModelDelayLoad> =
        sym!("OcrInitOptionsSetUseModelDelayLoad", OcrInitOptionsSetUseModelDelayLoad);
    let create_pipeline: Symbol<CreateOcrPipeline> =
        sym!("CreateOcrPipeline", CreateOcrPipeline);

    let create_proc_opts: Symbol<CreateOcrProcessOptions> =
        sym!("CreateOcrProcessOptions", CreateOcrProcessOptions);
    let set_max_lines: Symbol<OcrProcessOptionsSetMaxRecognitionLineCount> =
        sym!("OcrProcessOptionsSetMaxRecognitionLineCount", OcrProcessOptionsSetMaxRecognitionLineCount);

    let run_pipeline: Symbol<RunOcrPipeline> =
        sym!("RunOcrPipeline", RunOcrPipeline);

    let get_line_count: Symbol<GetOcrLineCount> =
        sym!("GetOcrLineCount", GetOcrLineCount);
    let get_line: Symbol<GetOcrLine> =
        sym!("GetOcrLine", GetOcrLine);
    let get_line_content: Symbol<GetOcrLineContent> =
        sym!("GetOcrLineContent", GetOcrLineContent);

    let mut ctx = 0i64;
    (create_init)(&mut ctx);
    (set_delay)(ctx, 0);

    let model = CString::new("oneocr.onemodel")?;
    let key = CString::new("kj)TGtrK>f]b[Piow.gU+nC@s\"\"\"\"\"\"4")?;

    let mut pipeline = 0i64;
    (create_pipeline)(
        model.as_ptr() as i64,
        key.as_ptr() as i64,
        ctx,
        &mut pipeline,
    );

    let mut opts = 0i64;
    (create_proc_opts)(&mut opts);
    (set_max_lines)(opts, 1000);

    let mut instance = 0i64;
    (run_pipeline)(pipeline, img, opts, &mut instance);

    let mut line_count = 0i64;
    (get_line_count)(instance, &mut line_count);

    for i in 0..line_count {
        let mut line = 0i64;
        (get_line)(instance, i, &mut line);
        if line == 0 {
            continue;
        }

        let mut text_ptr = 0i64;
        (get_line_content)(line, &mut text_ptr);
        if text_ptr == 0 {
            continue;
        }

        let cstr = CStr::from_ptr(text_ptr as *const c_char);
        println!("{}", cstr.to_string_lossy());
    }

    Ok(())
}}

/* Main */

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .context("Usage: ocr.exe <image.png>")?;

    let (img, _buffer) = load_image_bgra(&path)?;

    unsafe {
        run_ocr(&img)?;
    }

    Ok(())
}
