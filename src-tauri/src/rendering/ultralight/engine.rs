use std::{
    ffi::{c_ulonglong, c_void, CStr, CString},
    mem,
    path::PathBuf,
    ptr::null_mut,
    slice,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
};

use dcp::*;
use dcv_color_primitives as dcp;
use glium::{
    backend::Facade,
    glutin::{platform::windows::RawContextExt, ContextBuilder, NotCurrent},
    HeadlessRenderer,
};
use once_cell::sync::Lazy;
use ul_sys::*;
use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;

use self::driver::{
    gpu::{Bitmap, GPUCommand, GPUDriver, OwnedBitmap},
    GPUDriverReceiver, GPUDriverSender,
};

#[path = "./driver.rs"]
mod driver;

static END_WAIT_LOOP: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
static GPU_SENDER: Lazy<Mutex<Option<GPUDriverSender>>> = Lazy::new(|| Mutex::new(None));

pub struct Ultralight {
    renderer: ULRenderer,
    view: ULView,
    // bitmap: ULBitmap,
    driver_recv: GPUDriverReceiver,
}

impl Ultralight {
    #[allow(unused_mut)]
    pub fn new(app_folder: PathBuf) -> Ultralight {
        let mut renderer;
        let mut view;
        // let mut surface;
        // let mut bitmap;

        let (tx, rx) = mpsc::channel();

        let driver_sender = GPUDriverSender::new(0, 0, 0, tx);
        *GPU_SENDER.lock().unwrap() = Some(driver_sender);

        let ctx = unsafe { ContextBuilder::new().build_raw_context(GetDesktopWindow().0) }.unwrap();

        let gl_ctx =
            HeadlessRenderer::new::<NotCurrent>(unsafe { mem::transmute_copy(ctx.context()) })
                .unwrap();

        let driver_recv = GPUDriverReceiver::new(rx, gl_ctx.get_context()).unwrap();

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
            // ulPlatformSetLogger(ULLogger {
            //     log_message: Some(logger_callback),
            // });
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

            // surface = ulViewGetSurface(view);
            // bitmap = ulBitmapSurfaceGetBitmap(surface);
        };

        dcp::initialize();

        return Ultralight {
            renderer,
            view,
            // bitmap,
            driver_recv,
        };
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

    pub fn update(&self) {
        unsafe {
            ulUpdate(self.renderer);
        }
    }

    pub fn render(&mut self) {
        unsafe {
            ulRender(self.renderer);
        }
        let _ = self.driver_recv.render();
    }

    pub fn get_bitmap(&self) -> Result<Vec<u8>, &'static str> {
        // let mut bitmap: Vec<u8> = vec![0u8; 480 * 480 * 4];

        // let src_format = ImageFormat {
        //     pixel_format: PixelFormat::Bgra,
        //     color_space: ColorSpace::Rgb,
        //     num_planes: 1,
        // };

        // unsafe {
        //     let pixel_buf = ulBitmapLockPixels(self.bitmap);
        //     let pixel_buf_size = ulBitmapGetSize(self.bitmap);
        //     std::slice::from_raw_parts(pixel_buf as *mut u8, pixel_buf_size as usize)
        //         .clone_into(&mut bitmap);
        //     ulBitmapUnlockPixels(self.bitmap);
        // }

        // let dst_format = ImageFormat {
        //     pixel_format: PixelFormat::Rgb,
        //     color_space: ColorSpace::Rgb,
        //     num_planes: 1,
        // };

        // let mut dst = vec![0u8; 480 * 480 * 3];

        // let src_bgra_buf = &[&bitmap[..]];
        // let dst_rgb_buf = &mut [&mut dst[..]];

        // match convert_image(
        //     480,
        //     480,
        //     &src_format,
        //     None,
        //     src_bgra_buf,
        //     &dst_format,
        //     None,
        //     dst_rgb_buf,
        // ) {
        //     Err(err) => {
        //         println!("{:?}", err);
        //         return Err("Failed to convert to RGB.");
        //     }
        //     _ => {}
        // };

        // Ok(dst)
        Ok(vec![])
    }

    pub fn garbage_collect(&self) {
        unsafe {
            let context = ulViewLockJSContext(self.view);
            JSGarbageCollect(context);
            ulViewLockJSContext(self.view);
        }
    }

    pub fn call_js_script(&self, script: String) {
        let cstr = CString::new(script).unwrap();
        unsafe {
            let jsstr = JSStringCreateWithUTF8CString(cstr.as_ptr());

            let context = ulViewLockJSContext(self.view);

            let _ = JSEvaluateScript(context, jsstr, null_mut(), null_mut(), 1, null_mut());

            ulViewLockJSContext(self.view);

            JSStringRelease(jsstr);
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

pub unsafe extern "C" fn logger_callback(_log_level: ULLogLevel, message: ULString) {
    let message = CStr::from_ptr(message as *const i8);
    println!("{:?}", message);
}

pub unsafe extern "C" fn begin_sync() {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().begin_synchronize();
}

pub unsafe extern "C" fn end_sync() {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().end_synchronize();
}

pub unsafe extern "C" fn next_tex_id() -> u32 {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().next_texture_id()
}

pub unsafe extern "C" fn create_tex(texture_id: u32, bitmap: ULBitmap) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().create_texture(
        texture_id,
        OwnedBitmap::from_bitmap(&mut Bitmap::from_raw(bitmap).unwrap()).unwrap(),
    );
}

pub unsafe extern "C" fn update_tex(texture_id: u32, bitmap: ULBitmap) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().update_texture(
        texture_id,
        OwnedBitmap::from_bitmap(&mut Bitmap::from_raw(bitmap).unwrap()).unwrap(),
    );
}

pub unsafe extern "C" fn destroy_tex(texture_id: u32) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().destroy_texture(texture_id);
}

pub unsafe extern "C" fn next_render_buf_id() -> u32 {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().next_render_buffer_id()
}

pub unsafe extern "C" fn create_render_buf(render_buf_id: u32, buf: ULRenderBuffer) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender
        .unwrap()
        .create_render_buffer(render_buf_id, buf.into());
}

pub unsafe extern "C" fn destroy_render_buf(buf_id: u32) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().destroy_render_buffer(buf_id);
}

pub unsafe extern "C" fn next_geo_id() -> u32 {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().next_geometry_id()
}

pub unsafe extern "C" fn create_geo(geo_id: u32, vertices: ULVertexBuffer, indices: ULIndexBuffer) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender
        .unwrap()
        .create_geometry(geo_id, vertices.try_into().unwrap(), indices.into());
}

pub unsafe extern "C" fn update_geo(geo_id: u32, vertices: ULVertexBuffer, indices: ULIndexBuffer) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender
        .unwrap()
        .update_geometry(geo_id, vertices.try_into().unwrap(), indices.into());
}

pub unsafe extern "C" fn destroy_geo(geo_id: u32) {
    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().destroy_geometry(geo_id);
}

pub unsafe extern "C" fn update_cmd_lst(list: ULCommandList) {
    let commands_slice =
        unsafe { slice::from_raw_parts(list.commands, list.size.try_into().unwrap()) };

    let cmds = commands_slice
        .iter()
        .map(|cmd| GPUCommand::try_from(*cmd).unwrap())
        .collect();

    let mut sender_opt = GPU_SENDER.lock().unwrap();
    let sender = sender_opt.as_mut();
    sender.unwrap().update_command_list(cmds);
}
