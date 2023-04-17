use egui_backend::{BackendSettings, egui, GfxApiType, GfxBackend, NativeGlConfig, UserApp, WindowBackend};
use egui_backend::egui::RawInput;
use egui_render_wgpu::WgpuBackend;
use egui_window_glfw_passthrough::{glfw, GlfwConfig, GlfwWindow};
use egui_window_glfw_passthrough::glfw::{ClientApiHint, Context, OpenGlProfileHint, PixelImage, SwapInterval, WindowHint};
use crate::common;

struct Application<T: FnMut(&egui::Context)> {
    main_loop_logic: T,
}

impl<T: FnMut(&egui::Context)> Application<T> {
    fn new(main_loop_logic: T) -> Self {
        Application { main_loop_logic }
    }
}

impl<T: FnMut(&egui::Context)> UserApp<GlfwWindow, WgpuBackend> for Application<T> {
    fn run(&mut self, ctx: &egui::Context, glfw_backend: &mut GlfwWindow, _: &mut WgpuBackend) {
        glfw_backend.window.set_mouse_passthrough(true);
        common::gui::init_frame(ctx);
        (self.main_loop_logic)(ctx);
    }
}

pub fn run<T: FnMut(&egui::Context) + 'static>(main_loop_logic: T) {
    let mut glfw_backend = create_overlay(Default::default(), Default::default());
    let wgpu_backend = WgpuBackend::new(&mut glfw_backend, Default::default());
    let application = Application::new(main_loop_logic);
    glfw_backend.run_event_loop(wgpu_backend, application);
}

fn create_overlay(config: GlfwConfig, backend_settings: BackendSettings) -> GlfwWindow {
    let mut glfw_context =
        glfw::init(glfw::FAIL_ON_ERRORS).expect("failed to create glfw context");
    if let Some(glfw_callback) = config.glfw_callback {
        glfw_callback(&mut glfw_context);
    }
    // set hints based on gfx api config
    let mut swap_interval = None;
    let mut opengl = false;
    // set hints based on gfx api config
    match backend_settings.gfx_api_type.clone() {
        GfxApiType::OpenGL { native_config } => {
            opengl = true;
            let NativeGlConfig {
                major,
                minor,
                es,
                core,
                depth_bits,
                stencil_bits,
                samples,
                srgb,
                double_buffer,
                vsync,
                debug,
            } = native_config;
            if let Some(major) = major {
                glfw_context.window_hint(WindowHint::ContextVersionMajor(major.into()));
            }
            if let Some(value) = minor {
                glfw_context.window_hint(WindowHint::ContextVersionMinor(value.into()));
            }
            if let Some(value) = es {
                glfw_context.window_hint(WindowHint::ClientApi(if value {
                    ClientApiHint::OpenGlEs
                } else {
                    ClientApiHint::OpenGl
                }));
            }
            if let Some(value) = core {
                glfw_context.window_hint(WindowHint::OpenGlProfile(if value {
                    glfw::OpenGlProfileHint::Core
                } else {
                    OpenGlProfileHint::Compat
                }));
            }

            glfw_context.window_hint(WindowHint::DepthBits(depth_bits.map(Into::into)));

            glfw_context.window_hint(WindowHint::StencilBits(stencil_bits.map(Into::into)));

            if let Some(srgb) = srgb {
                glfw_context.window_hint(WindowHint::SRgbCapable(srgb));
            }
            if let Some(samples) = samples {
                glfw_context.window_hint(WindowHint::Samples(Some(samples as u32)));
            }
            if let Some(value) = double_buffer {
                glfw_context.window_hint(WindowHint::DoubleBuffer(value.into()));
            }
            swap_interval = vsync;

            if let Some(debug) = debug {
                glfw_context.window_hint(WindowHint::OpenGlDebugContext(debug));
            }
        }
        GfxApiType::NoApi => {
            glfw_context.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        }
        GfxApiType::Vulkan => {
            glfw_context.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        }
    }

    // hints for passthrough and overlay features
    glfw_context.window_hint(glfw::WindowHint::Decorated(false));
    glfw_context.window_hint(glfw::WindowHint::Focused(false));
    glfw_context.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    glfw_context.window_hint(glfw::WindowHint::MousePassthrough(true));
    glfw_context.window_hint(glfw::WindowHint::Floating(true));
    glfw_context.window_hint(glfw::WindowHint::ScaleToMonitor(true));
    // create a window
    let (mut window, events_receiver) = glfw_context.with_primary_monitor(|glfw_context, m| {
        let monitor = m.expect("Primary monitor not found");
        let mode = monitor.get_video_mode().unwrap();
        glfw_context.create_window(mode.width, mode.height, "QUIN", glfw::WindowMode::Windowed)
    }).expect("Failed to create GLFW window");

    // iconify
    let image = image::load_from_memory(include_bytes!("../assets/icon.png"))
        .expect("Failed to load image.")
        .to_rgba8();
    let icon = PixelImage {
        width: image.width(),
        height: image.height(),
        pixels: image.into_raw().chunks(4)
            .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .collect(),
    };
    window.set_icon_from_pixels(vec![icon]);

    // set which events you care about
    window.set_all_polling(true);
    window.set_store_lock_key_mods(true);
    if opengl {
        window.make_current();

        if let Some(value) = swap_interval {
            glfw_context.set_swap_interval(if value {
                SwapInterval::Sync(1)
            } else {
                SwapInterval::None
            });
        }
    }
    // collect details and keep them updated
    let (width, height) = window.get_framebuffer_size();
    let (logical_width, _) = window.get_size();
    let scale = width as f32 / logical_width as f32;
    let cursor_position = window.get_cursor_pos();
    let size_physical_pixels = [
        width.try_into().expect("width not fit in u32"),
        height.try_into().expect("height not fit in u32"),
    ];
    let raw_input = RawInput {
        screen_rect: Some(egui::Rect::from_points(&[
            Default::default(),
            [width as f32, height as f32].into(),
        ])),
        pixels_per_point: Some(scale),
        ..Default::default()
    };

    GlfwWindow {
        glfw: glfw_context,
        events_receiver,
        window,
        size_physical_pixels,
        scale,
        cursor_pos_physical_pixels: [cursor_position.0 as f32, cursor_position.1 as f32],
        raw_input,
        frame_events: vec![],
        resized_event_pending: true,
        backend_settings,
    }
}
