use super::BACKEND;
use crate::hal::native::{shader_strings, WrappedContext};
use crate::hal::scaler::ScreenScaler;
use crate::hal::{setup_quad, Framebuffer, Shader};
use crate::prelude::{BTerm, InitHints, BACKEND_INTERNAL};
use crate::BResult;
use glow::HasContext;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    platform_hints: InitHints,
) -> BResult<BTerm> {
    let mut scaler = ScreenScaler::new(platform_hints.desired_gutter, width_pixels, height_pixels);
    let el = EventLoop::new().map_err(|e| format!("Failed to create event loop: {}", e))?;
    let window_size = scaler.new_window_size();
    let window_size = winit::dpi::LogicalSize::new(window_size.width, window_size.height);

    let window_attributes = Window::default_attributes()
        .with_title(window_title.to_string())
        .with_resizable(platform_hints.fitscreen)
        .with_min_inner_size(window_size)
        .with_inner_size(window_size);

    // Create config template for OpenGL
    let template_builder = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .prefer_hardware_accelerated(Some(true))
        .with_transparency(false);

    // Use DisplayBuilder the simple way - it creates both display and window
    let (window, gl_config) = DisplayBuilder::new()
        .with_window_attributes(Some(window_attributes))
        .build(&el, template_builder, |configs| {
            configs
                .reduce(|accum, config| {
                    let transparency_check =
                        config.supports_transparency().unwrap_or(false) == false;
                    let accumulate =
                        accum.num_samples() < config.num_samples() && transparency_check;
                    if accumulate {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .map_err(|e| format!("Failed to build display: {}", e))?;

    let window = std::rc::Rc::new(window.ok_or("Failed to create window")?);

    // Get the display from the config
    let gl_display = gl_config.display();
    let raw_window_handle = window
        .raw_window_handle()
        .map_err(|e| format!("Failed to get raw window handle: {}", e))?;
    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(glutin::context::ContextApi::OpenGl(Some(
            glutin::context::Version::new(platform_hints.opengl_major, platform_hints.opengl_minor),
        )))
        .with_profile(if platform_hints.opengl_core {
            glutin::context::GlProfile::Core
        } else {
            glutin::context::GlProfile::Compatibility
        })
        .build(Some(raw_window_handle));

    let gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .map_err(|e| format!("Failed to create GL context: {}", e))?
    };

    // Create surface
    let (width, height): (u32, u32) = window.inner_size().into();
    let raw_window_handle_surface = window
        .raw_window_handle()
        .map_err(|e| format!("Failed to get raw window handle for surface: {}", e))?;
    let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
        .build(
            raw_window_handle_surface,
            std::num::NonZeroU32::new(width).unwrap(),
            std::num::NonZeroU32::new(height).unwrap(),
        );

    let gl_surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .map_err(|e| format!("Failed to create surface: {}", e))?
    };

    // Make context current
    let gl_context = gl_context
        .make_current(&gl_surface)
        .map_err(|e| format!("Failed to make context current: {}", e))?;

    if platform_hints.fullscreen {
        if let Some(mh) = window.available_monitors().next() {
            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(mh))));
        } else {
            return Err("No available monitor found".into());
        }
    }

    let gl = unsafe {
        glow::Context::from_loader_function(|s| {
            let c_str = std::ffi::CString::new(s).unwrap();
            gl_display.get_proc_address(&c_str) as *const _
        })
    };

    #[cfg(debug_assertions)]
    unsafe {
        let gl_version = gl.get_parameter_string(glow::VERSION);
        let shader_version = gl.get_parameter_string(glow::SHADING_LANGUAGE_VERSION);
        println!(
            "Initialized OpenGL with: {}, Shader Language Version: {}",
            gl_version, shader_version
        );
    }

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_WITH_BG_VS,
        shader_strings::CONSOLE_WITH_BG_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_NO_BG_VS,
        shader_strings::CONSOLE_NO_BG_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::BACKING_VS,
        shader_strings::BACKING_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SCANLINES_VS,
        shader_strings::SCANLINES_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::FANCY_CONSOLE_VS,
        shader_strings::FANCY_CONSOLE_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SPRITE_CONSOLE_VS,
        shader_strings::SPRITE_CONSOLE_FS,
    ));

    // Build the backing frame-buffer
    let initial_dpi_factor = window.scale_factor();
    scaler.change_logical_size(width_pixels, height_pixels, initial_dpi_factor as f32);
    let backing_fbo = Framebuffer::build_fbo(
        &gl,
        scaler.logical_size.0 as i32,
        scaler.logical_size.1 as i32,
    )?;

    // Build a simple quad rendering VAO
    let quad_vao = setup_quad(&gl);

    let mut be = BACKEND.lock();
    be.gl = Some(gl);
    be.quad_vao = Some(quad_vao);
    be.context_wrapper = Some(WrappedContext {
        el,
        window: window.clone(),
        gl_context,
        gl_surface,
        gl_display,
    });
    be.backing_buffer = Some(backing_fbo);
    be.frame_sleep_time = crate::hal::convert_fps_to_wait(platform_hints.frame_sleep_time);
    be.resize_scaling = platform_hints.resize_scaling;
    be.screen_scaler = scaler;

    BACKEND_INTERNAL.lock().shaders = shaders;

    let bterm = BTerm {
        width_pixels,
        height_pixels,
        original_width_pixels: width_pixels,
        original_height_pixels: height_pixels,
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
        screen_burn_color: bracket_color::prelude::RGB::from_f32(0.0, 1.0, 1.0),
        mouse_visible: true,
    };
    Ok(bterm)
}
