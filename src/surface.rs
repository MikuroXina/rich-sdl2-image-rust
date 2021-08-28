//! Image surface to load some image format to [`Surface`].

use rich_sdl2_rust::{
    surface::{RawSurface, Surface},
    Result, Sdl, SdlError,
};
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use crate::bind;

/// An image surface for the loaded picture.
pub struct ImgSurface {
    surface: NonNull<RawSurface>,
}

impl ImgSurface {
    /// Constructs a new image surface from the file. The format will automatically determined if `file_type` is `None`. `file_type` is case-insensitive and can be one of these:
    ///
    /// - `"TGA"`
    /// - `"CUR"`
    /// - `"ICO"`
    /// - `"BMP"`
    /// - `"GIF"`
    /// - `"JPG"`
    /// - `"LBM"`
    /// - `"PCX"`
    /// - `"PNG"`
    /// - `"PNM"`
    /// - `"SVG"`
    /// - `"TIF"`
    /// - `"XCF"`
    /// - `"XPM"`
    /// - `"XV"`
    /// - `"WEBP"`
    ///
    /// # Panics
    ///
    /// Panics if `file_name` or `file_type` is an empty string.
    pub fn new(file_name: &str, file_type: Option<&str>) -> Result<Self> {
        let file_name_cstr = CString::new(file_name).expect("file_name mus not be empty");
        let mode = CStr::from_bytes_with_nul(b"rb\0").unwrap();
        let fp = unsafe { bind::SDL_RWFromFile(file_name_cstr.as_ptr(), mode.as_ptr()) };
        let file_type_cstr =
            file_type.map(|file_type| CString::new(file_type).expect("file_name mus not be empty"));
        let ptr = unsafe {
            bind::IMG_LoadTyped_RW(
                fp,
                1,
                file_type_cstr.map_or(std::ptr::null(), |cstr| cstr.as_ptr()),
            )
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                surface: NonNull::new(ptr.cast()).unwrap(),
            })
        }
    }
}

impl Surface for ImgSurface {
    fn as_ptr(&self) -> NonNull<RawSurface> {
        self.surface
    }
}

impl Drop for ImgSurface {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.surface.as_ptr().cast()) }
    }
}
