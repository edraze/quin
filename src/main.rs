use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use egui_backend::{BackendSettings, egui, GfxApiType, GfxBackend, NativeGlConfig, UserApp, WindowBackend};
use egui_backend::egui::{Color32, Pos2, RawInput};
use egui_render_wgpu::WgpuBackend;
use egui_window_glfw_passthrough::glfw::{ClientApiHint, Context, OpenGlProfileHint, SwapInterval, WindowHint, WindowMode};
use egui_window_glfw_passthrough::{glfw, GlfwConfig, GlfwWindow};
use rdev::{Event, EventType, Key};


#[derive(Default)]
struct Point {
    x: f32,
    y: f32,
    key: String,
}

struct Application {
    grid_mode_active: bool,
    points: Vec<Point>,
    rchan: Receiver<Event>,
}

impl Application {
    fn new(rchan: Receiver<Event>) -> Self {
        Application { grid_mode_active: false, points: Vec::new(), rchan }
    }

    fn draw_grid(&mut self, ctx: &egui::Context) {
        if self.grid_mode_active {
            let panel_frame = egui::Frame {
                fill: Color32::TRANSPARENT,
                rounding: 0.0.into(),
                stroke: egui::Stroke::none(),
                outer_margin: 0.0.into(),
                ..Default::default()
            };

            egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
                if self.points.is_empty() {
                    let step = 30.0;
                    let of_set = 10.0;
                    let (w, h) = rdev::display_size().unwrap();
                    let h_dots = w / step as u64;
                    let v_dots = h / step as u64;
                    for v_index in 1..=v_dots {
                        for h_index in 1..=h_dots {
                            let x = h_index as f32 * step - of_set;
                            let y = v_index as f32 * step - of_set;
                            ui.painter().add(egui::epaint::CircleShape::filled(Pos2::new(x, y), 0.6, Color32::GREEN));
                            self.points.push(Point { x, y, key: (v_index + h_index).to_string() })
                        }
                    }
                } else {
                    for point in &self.points {
                        ui.painter().add(egui::epaint::CircleShape::filled(Pos2::new(point.x, point.y), 0.6, Color32::GREEN));
                    }
                }
            });
        } else {
            let panel_frame = egui::Frame {
                fill: Color32::TRANSPARENT,
                rounding: 0.0.into(),
                stroke: egui::Stroke::none(),
                outer_margin: 0.0.into(),
                ..Default::default()
            };
            egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
                ui.painter().add(egui::epaint::CircleShape::filled(Pos2::new(0.0, 0.0), 0.6, Color32::BLACK));
            });
        }
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        if let Ok(event) = self.rchan.try_recv() {
            let event_type = event.event_type;
            if event_type == EventType::KeyRelease(Key::AltGr) {
                println!("Received {:?}", event_type);
                self.grid_mode_active = !self.grid_mode_active;
                self.draw_grid(ctx);
            }
        }
    }
}

impl UserApp<GlfwWindow, WgpuBackend> for Application {
    fn run(&mut self, ctx: &egui::Context, glfw_backend: &mut GlfwWindow, _: &mut WgpuBackend) {
        glfw_backend.window.set_mouse_passthrough(true);
        self.draw_grid(ctx);
        self.handle_input(ctx);
    }
}

pub fn start_overlay(app: impl UserApp<GlfwWindow, WgpuBackend> + 'static) {
    let mut glfw_backend = new_window(Default::default(), Default::default());
    glfw_backend.glfw.window_hint(WindowHint::Floating(true));
    let wgpu_backend = WgpuBackend::new(&mut glfw_backend, Default::default());

    glfw_backend.run_event_loop(wgpu_backend, app);
}

fn new_window(config: GlfwConfig, backend_settings: BackendSettings) -> GlfwWindow {
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
        _ => {}
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
        // set raw input screen rect details so that first frame
        // will have correct size even without any resize event
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
        backend_settings, // provide so that on first prepare frame, renderers can set their viewport sizes
    }
}

fn main() {
    let (schan, mut rchan) = mpsc::channel();
    thread::spawn(move || {
        rdev::listen(move |event| {
            schan
                .send(event)
                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
        }).expect("Could not listen");
    });

    start_overlay(Application::new(rchan));
}
