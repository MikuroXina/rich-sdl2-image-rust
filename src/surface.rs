//! Image surface to load some image format to [`Surface`].

use rich_sdl2_rust::{
    surface::{RawSurface, Surface},
    Result, Sdl, SdlError,
};
use std::{ffi::CString, ptr::NonNull};

use crate::bind;

/// An image surface for the loaded picture.
pub struct ImgSurface {
    surface: NonNull<RawSurface>,
}

impl ImgSurface {
    /// Constructs a new image surface from the file.
    pub fn new(file_name: &str) -> Result<Self> {
        let cstr = CString::new(file_name).expect("file_name mus not be empty");
        let ptr = unsafe { bind::IMG_Load(cstr.as_ptr()) };
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
