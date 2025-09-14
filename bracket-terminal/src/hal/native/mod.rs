mod init;
pub mod shader_strings;
use glow::NativeVertexArray;
pub use init::*;
mod mainloop;
use crate::hal::scaler::{default_gutter_size, ScreenScaler};
use crate::hal::ConsoleBacking;
pub use mainloop::*;
use parking_lot::Mutex;
use std::any::Any;

pub type GlCallback = fn(&mut dyn Any, &glow::Context);

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        gl: None,
        quad_vao: None,
        context_wrapper: None,
        backing_buffer: None,
        frame_sleep_time: None,
        gl_callback: None,
        resize_scaling: false,
        resize_request: None,
        request_screenshot: None,
        screen_scaler: ScreenScaler::default(),
    });
}

lazy_static! {
    pub(crate) static ref CONSOLE_BACKING: Mutex<Vec<ConsoleBacking>> = Mutex::new(Vec::new());
}

pub struct PlatformGL {
    pub gl: Option<glow::Context>,
    pub quad_vao: Option<NativeVertexArray>,
    pub context_wrapper: Option<WrappedContext>,
    pub backing_buffer: Option<super::Framebuffer>,
    pub frame_sleep_time: Option<u64>,
    pub gl_callback: Option<GlCallback>,
    pub resize_scaling: bool,
    pub resize_request: Option<(u32, u32)>,
    pub request_screenshot: Option<String>,
    pub screen_scaler: ScreenScaler,
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

pub struct WrappedContext {
    pub el: winit::event_loop::EventLoop<()>,
    pub window: std::rc::Rc<winit::window::Window>,
    pub gl_context: glutin::context::PossiblyCurrentContext,
    pub gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    pub gl_display: glutin::display::Display,
}

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
    pub opengl_major: u8,
    pub opengl_minor: u8,
    pub opengl_core: bool,
    pub hardware_acceleration: bool,
    pub srgb: bool,
    pub frame_sleep_time: Option<f32>,
    pub resize_scaling: bool,
    pub desired_gutter: u32,
    pub fitscreen: bool,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: false,
            fullscreen: false,
            opengl_major: 3,
            opengl_minor: 2,
            opengl_core: true,
            hardware_acceleration: false,
            srgb: true,
            frame_sleep_time: None,
            resize_scaling: false,
            desired_gutter: default_gutter_size(),
            fitscreen: false,
        }
    }
}

impl Default for InitHints {
    fn default() -> Self {
        Self {
            vsync: false,
            fullscreen: false,
            opengl_major: 3,
            opengl_minor: 2,
            opengl_core: true,
            hardware_acceleration: false,
            srgb: true,
            frame_sleep_time: None,
            resize_scaling: false,
            desired_gutter: default_gutter_size(),
            fitscreen: false,
        }
    }
}

pub fn log(s: &str) {
    println!("{}", s);
}
