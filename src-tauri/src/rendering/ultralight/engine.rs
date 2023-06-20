use std::{
    ffi::{c_ulonglong, c_void, CString},
    ptr::null_mut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use dcp::*;
use dcv_color_primitives as dcp;
use once_cell::sync::Lazy;
use ul_sys::*;

use crate::APP_FOLDER;

static END_WAIT_LOOP: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

pub struct Ultralight {
    renderer: ULRenderer,
    view: ULView,
    bitmap: ULBitmap,
}

impl Ultralight {
    #[allow(unused_mut)]
    pub fn new() -> Ultralight {
        let mut renderer;
        let mut view;
        let mut surface;
        let mut bitmap;

        unsafe {
            let config = ulCreateConfig();

            ulPlatformSetClipboard(ULClipboard {
                clear: None,
                read_plain_text: None,
                write_plain_text: None,
            });

            let mut log_path = APP_FOLDER.clone();
            log_path.push("log.txt");

            let log_path_cs = CString::new(log_path.to_str().unwrap()).unwrap();
            let log_path_ul = ulCreateString(log_path_cs.as_ptr());
            ulEnableDefaultLogger(log_path_ul);
            ulDestroyString(log_path_ul);
            // ulPlatformSetLogger(ULLogger {
            // log_message: Some(logger_callback),
            // });
            ulEnablePlatformFontLoader();

            let fs_folder = CString::new(APP_FOLDER.to_str().unwrap()).unwrap();
            let fs_folder_ul = ulCreateString(fs_folder.as_ptr());
            ulEnablePlatformFileSystem(fs_folder_ul);
            ulDestroyString(fs_folder_ul);

            renderer = ulCreateRenderer(config);

            let view_config = ulCreateViewConfig();

            view = ulCreateView(renderer, 480, 480, view_config, null_mut());
            ulViewSetFinishLoadingCallback(view, Some(finished_callback), null_mut());
            // let default_html = CString::new("http://localhost:2137").unwrap();
            // let default_html_ul = ulCreateString(default_html.as_ptr());
            // ulViewLoadHTML(view, default_html_ul);
            // ulDestroyString(default_html_ul);

            // while !END_WAIT_LOOP.load(Ordering::Acquire) {
            //     ulUpdate(renderer);
            //     ulRender(renderer);
            // }
            // END_WAIT_LOOP.store(false, Ordering::Relaxed);

            surface = ulViewGetSurface(view);
            bitmap = ulBitmapSurfaceGetBitmap(surface);
        };

        dcp::initialize();

        return Ultralight {
            renderer,
            view,
            bitmap,
        };
    }

    // pub fn load_html(&self, html: &str) -> Result<(), &'static str> {
    //     END_WAIT_LOOP.store(false, Ordering::Relaxed);

    //     let html_cstring = match CString::new(html) {
    //         Err(_) => return Err("Failed to create CString. Is HTML valid?"),
    //         Ok(val) => val,
    //     };

    //     unsafe {
    //         let html_ul = ulCreateString(html_cstring.as_ptr());

    //         ulViewLoadHTML(self.view, html_ul);
    //         ulDestroyString(html_ul);

    //         while !END_WAIT_LOOP.load(Ordering::Acquire) {
    //             ulUpdate(self.renderer);
    //             ulRender(self.renderer);
    //         }
    //     }

    //     END_WAIT_LOOP.store(false, Ordering::Relaxed);

    //     Ok(())
    // }

    pub fn load_url(&mut self, html: &str) -> Result<(), &'static str> {
        END_WAIT_LOOP.store(false, Ordering::Relaxed);

        let url_cstring = match CString::new(html) {
            Err(_) => return Err("Failed to create CString. Is URL valid?"),
            Ok(val) => val,
        };

        unsafe {
            let url_ul = ulCreateString(url_cstring.as_ptr());

            ulViewLoadURL(self.view, url_ul);
            ulDestroyString(url_ul);

            while !END_WAIT_LOOP.load(Ordering::Acquire) {
                ulUpdate(self.renderer);
                ulRender(self.renderer);
            }
        }

        END_WAIT_LOOP.store(false, Ordering::Relaxed);

        Ok(())
    }

    pub fn update(&self) {
        unsafe {
            ulUpdate(self.renderer);
        }
    }

    pub fn render(&self) {
        unsafe {
            ulRender(self.renderer);
        }
    }

    pub fn get_bitmap(&self) -> Result<Vec<u8>, &'static str> {
        let mut bitmap: Vec<u8> = vec![0u8; 480 * 480 * 4];

        let src_format = ImageFormat {
            pixel_format: PixelFormat::Bgra,
            color_space: ColorSpace::Rgb,
            num_planes: 1,
        };

        unsafe {
            let pixel_buf = ulBitmapLockPixels(self.bitmap);
            let pixel_buf_size = ulBitmapGetSize(self.bitmap);
            std::slice::from_raw_parts(pixel_buf as *mut u8, pixel_buf_size as usize)
                .clone_into(&mut bitmap);
            ulBitmapUnlockPixels(self.bitmap);
        }

        let dst_format = ImageFormat {
            pixel_format: PixelFormat::Rgb,
            color_space: ColorSpace::Rgb,
            num_planes: 1,
        };

        let mut dst = vec![0u8; 480 * 480 * 3];

        let src_bgra_buf = &[&bitmap[..]];
        let dst_rgb_buf = &mut [&mut dst[..]];

        match convert_image(
            480,
            480,
            &src_format,
            None,
            src_bgra_buf,
            &dst_format,
            None,
            dst_rgb_buf,
        ) {
            Err(err) => {
                println!("{:?}", err);
                return Err("Failed to convert to RGB.");
            }
            _ => {}
        };

        Ok(dst)
    }

    pub fn garbage_collect(&self) {
        unsafe {
            let context = ulViewLockJSContext(self.view);
            JSGarbageCollect(context);
            ulViewLockJSContext(self.view);
        }
    }
}

#[allow(unused_variables)]
pub unsafe extern "C" fn finished_callback(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
) {
    if is_main_frame {
        END_WAIT_LOOP.store(true, Ordering::Relaxed);
        println!("Ready to render.");
    }
}

// pub unsafe extern "C" fn logger_callback(log_level: ULLogLevel, message: ULString) {
//     let message = CString::from_raw(ulStringGetData(message) as *mut i8);
//     println!("{:?}", message);
// }
