use std::{
    borrow::Cow,
    cell::RefCell,
    ffi::{c_ulonglong, c_void, CString},
    path::PathBuf,
    ptr::null_mut,
    slice,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use heapless::spsc::{Consumer, Producer, Queue};

use glium::{buffer::ReadMapping, pixel_buffer::PixelBuffer, texture::RawImage2d};
use rayon::prelude::*;

use once_cell::sync::Lazy;
use ul_sys::*;

use self::driver::{
    gpu::{Bitmap, GPUCommand, GPUDriver, OwnedBitmap},
    GPUDriverCommand, GPUDriverReceiver, GPUDriverSender,
};

static mut QUEUE: Queue<GPUDriverCommand, 32> = Queue::new();

#[path = "./driver.rs"]
mod driver;

static END_WAIT_LOOP: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
static mut GPU_SENDER: Lazy<GPUDriverSender> = Lazy::new(|| GPUDriverSender::new(0, 0, 0));

pub struct Ultralight {
    renderer: ULRenderer,
    view: ULView,
    driver_recv: GPUDriverReceiver,
}

impl Ultralight {
    #[allow(unused_mut)]
    pub fn new(app_folder: PathBuf) -> Ultralight {
        let mut renderer;
        let mut view;

        // unsafe { GPU_SENDER.set_tx(producer) };

        let driver_recv = GPUDriverReceiver::new(app_folder.clone()).unwrap();

        unsafe {
            let config = ulCreateConfig();

            ulPlatformSetGPUDriver(ULGPUDriver {
                begin_synchronize: Some(begin_sync),
                end_synchronize: Some(end_sync),
                next_texture_id: Some(next_tex_id),
                create_texture: Some(create_tex),
                update_texture: Some(update_tex),
                destroy_texture: Some(destroy_tex),
                next_render_buffer_id: Some(next_render_buf_id),
                create_render_buffer: Some(create_render_buf),
                destroy_render_buffer: Some(destroy_render_buf),
                next_geometry_id: Some(next_geo_id),
                create_geometry: Some(create_geo),
                update_geometry: Some(update_geo),
                destroy_geometry: Some(destroy_geo),
                update_command_list: Some(update_cmd_lst),
            });

            ulPlatformSetClipboard(ULClipboard {
                clear: None,
                read_plain_text: None,
                write_plain_text: None,
            });

            let mut log_path = app_folder.clone();
            log_path.push("log.txt");

            let log_path_cs = CString::new(log_path.to_str().unwrap()).unwrap();
            let log_path_ul = ulCreateString(log_path_cs.as_ptr());
            ulEnableDefaultLogger(log_path_ul);
            ulDestroyString(log_path_ul);
            ulPlatformSetLogger(ULLogger {
                log_message: Some(logger_callback),
            });
            ulEnablePlatformFontLoader();

            let fs_folder = CString::new(app_folder.to_str().unwrap()).unwrap();
            let fs_folder_ul = ulCreateString(fs_folder.as_ptr());
            ulEnablePlatformFileSystem(fs_folder_ul);
            ulDestroyString(fs_folder_ul);

            renderer = ulCreateRenderer(config);

            let view_config = ulCreateViewConfig();
            ulViewConfigSetIsAccelerated(view_config, true);

            view = ulCreateView(renderer, 480, 480, view_config, null_mut());
            ulViewSetFinishLoadingCallback(view, Some(finished_callback), null_mut());
        };

        Ultralight {
            renderer,
            view,
            driver_recv,
        }
    }

    #[allow(dead_code)]
    pub fn load_html(&self, html: &str) -> Result<(), &'static str> {
        END_WAIT_LOOP.store(false, Ordering::Relaxed);

        let html_cstring = match CString::new(html) {
            Err(_) => return Err("Failed to create CString. Is HTML valid?"),
            Ok(val) => val,
        };

        unsafe {
            let html_ul = ulCreateString(html_cstring.as_ptr());

            ulViewLoadHTML(self.view, html_ul);
            ulDestroyString(html_ul);

            while !END_WAIT_LOOP.load(Ordering::Acquire) {
                ulUpdate(self.renderer);
                ulRender(self.renderer);
            }
        }

        END_WAIT_LOOP.store(false, Ordering::Relaxed);

        Ok(())
    }

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

    #[inline(always)]
    pub fn update(&self) {
        unsafe {
            ulUpdate(self.renderer);
        }
    }

    #[inline(always)]
    pub fn render(&mut self) {
        unsafe {
            ulRender(self.renderer);
        }

        let _ = self.driver_recv.render();
    }

    #[inline]
    pub fn get_bitmap(&mut self) -> Result<Cow<'_, [u8]>, &'static str> {
        let render_target = unsafe { ulViewGetRenderTarget(self.view) };
        let src = self.driver_recv.render_bitmap(render_target.texture_id)?;

        Ok(src)
    }

    #[inline(always)]
    pub fn garbage_collect(&self) {
        unsafe {
            let context = ulViewLockJSContext(self.view);
            JSGarbageCollect(context);
            ulViewLockJSContext(self.view);
        }
    }

    #[inline]
    pub fn call_js_script(&self, script: String) {
        let cstr = CString::new(script).unwrap();
        unsafe {
            // let jsstr = JSStringCreateWithUTF8CString(cstr.as_ptr());

            // let context = ulViewLockJSContext(self.view);

            let ulstr = ulCreateString(cstr.as_ptr());

            ulViewEvaluateScript(self.view, ulstr, null_mut());

            ulDestroyString(ulstr);

            // let _ = JSEvaluateScript(context, jsstr, null_mut(), null_mut(), 1, null_mut());

            // ulViewLockJSContext(self.view);

            // JSStringRelease(jsstr);
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

pub unsafe extern "C" fn logger_callback(log_level: ULLogLevel, message: ULString) {
    let raw_data = ulStringGetData(message);
    let utf8_data = slice::from_raw_parts(raw_data, ulStringGetLength(message) as usize)
        .iter()
        .map(|c| *c as u8)
        .collect();

    let msg = String::from_utf8(utf8_data).map_err(|_| "X").unwrap();

    #[allow(non_upper_case_globals)]
    let log_level = match log_level {
        ULLogLevel_kLogLevel_Error => "Error",
        ULLogLevel_kLogLevel_Info => "Info",
        ULLogLevel_kLogLevel_Warning => "Warning",
        _ => "None",
    };

    println!("[ {log_level} ] {msg}");
}

pub unsafe extern "C" fn begin_sync() {
    GPU_SENDER.begin_synchronize();
}

pub unsafe extern "C" fn end_sync() {
    GPU_SENDER.end_synchronize();
}

pub unsafe extern "C" fn next_tex_id() -> u32 {
    GPU_SENDER.next_texture_id()
}

pub unsafe extern "C" fn create_tex(texture_id: u32, bitmap: ULBitmap) {
    GPU_SENDER.create_texture(
        texture_id,
        OwnedBitmap::from_bitmap(&mut Bitmap::from_raw(bitmap).unwrap()).unwrap(),
    );
}

pub unsafe extern "C" fn update_tex(texture_id: u32, bitmap: ULBitmap) {
    GPU_SENDER.update_texture(
        texture_id,
        OwnedBitmap::from_bitmap(&mut Bitmap::from_raw(bitmap).unwrap()).unwrap(),
    );
}

pub unsafe extern "C" fn destroy_tex(texture_id: u32) {
    GPU_SENDER.destroy_texture(texture_id);
}

pub unsafe extern "C" fn next_render_buf_id() -> u32 {
    GPU_SENDER.next_render_buffer_id()
}

pub unsafe extern "C" fn create_render_buf(render_buf_id: u32, buf: ULRenderBuffer) {
    GPU_SENDER.create_render_buffer(render_buf_id, buf.into());
}

pub unsafe extern "C" fn destroy_render_buf(buf_id: u32) {
    GPU_SENDER.destroy_render_buffer(buf_id);
}

pub unsafe extern "C" fn next_geo_id() -> u32 {
    GPU_SENDER.next_geometry_id()
}

pub unsafe extern "C" fn create_geo(geo_id: u32, vertices: ULVertexBuffer, indices: ULIndexBuffer) {
    GPU_SENDER.create_geometry(geo_id, vertices.try_into().unwrap(), indices.into());
}

pub unsafe extern "C" fn update_geo(geo_id: u32, vertices: ULVertexBuffer, indices: ULIndexBuffer) {
    GPU_SENDER.update_geometry(geo_id, vertices.try_into().unwrap(), indices.into());
}

pub unsafe extern "C" fn destroy_geo(geo_id: u32) {
    GPU_SENDER.destroy_geometry(geo_id);
}

pub unsafe extern "C" fn update_cmd_lst(list: ULCommandList) {
    let commands_slice =
        unsafe { slice::from_raw_parts(list.commands, list.size.try_into().unwrap()) };

    let cmds = commands_slice
        .iter()
        .map(|cmd| GPUCommand::try_from(*cmd).unwrap())
        .collect();

    GPU_SENDER.update_command_list(cmds);
}
