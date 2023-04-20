use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use egui_backend::{BackendSettings, egui, GfxApiType, GfxBackend, NativeGlConfig, UserApp, WindowBackend};
use egui_backend::egui::{Color32, FontFamily, FontId, Pos2, RawInput, Ui};
use egui_render_wgpu::WgpuBackend;
use egui_window_glfw_passthrough::glfw::{ClientApiHint, Context, OpenGlProfileHint, PixelImage, SwapInterval, WindowHint};
use egui_window_glfw_passthrough::{glfw, GlfwConfig, GlfwWindow};
use rdev::{Event, EventType, Key};
use rdev::Button::{Left, Middle, Right};

const KEY_LETTERS: [&str; 108] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
    "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54",
    "55", "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80", "81"];

#[derive(Default, Debug)]
struct Point {
    x: f32,
    y: f32,
    key: String,
}

struct Application {
    grid_mode_active: bool,
    points: Vec<Point>,
    rchan: Receiver<Event>,
    input_buffer: Vec<char>,
    backspace_inited: bool,
}

impl Application {
    fn new(rchan: Receiver<Event>) -> Self {
        Application { grid_mode_active: false, points: Vec::new(), rchan, input_buffer: Vec::new(), backspace_inited: false }
    }

    fn draw_empty(&self, ctx: &egui::Context) {
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

                    let mut v_keys_iter = KEY_LETTERS.iter();
                    for v_index in 1..=v_dots {
                        let v_key = v_keys_iter.next().unwrap();

                        let mut h_keys_iter = KEY_LETTERS.iter();
                        for h_index in 1..=h_dots {
                            let h_key = h_keys_iter.next().unwrap();
                            let x = h_index as f32 * step - of_set;
                            let y = v_index as f32 * step - of_set;
                            let key = format!("{}{}", v_key, h_key);
                            let point = Point { x, y, key };

                            self.draw_point(ui, &point);
                            self.points.push(point);
                        }
                    }
                } else {
                    for point in &self.points {
                        self.draw_point(ui, point);
                    }
                }
            });
        } else {
            self.draw_empty(ctx);
        }
    }

    fn draw_point(&self, ui: &mut Ui, point: &Point) {
        let position = Pos2::new(point.x, point.y);
        ui.painter().add(egui::epaint::CircleShape::filled(position, 0.6, Color32::RED));
        let painter = ui.painter();
        let galley = painter.layout(
            point.key.clone(),
            FontId::new(10.0, FontFamily::Proportional),
            Color32::YELLOW,
            f32::INFINITY,
        );
        painter.add(egui::epaint::TextShape::new(position, galley));
    }

    fn handle_input(&mut self) {
        if let Ok(event) = self.rchan.try_recv() {
            if let EventType::KeyRelease(key) = event.event_type {
                if key == Key::AltGr {
                    println!("Received key: {:?}", key);
                    self.grid_mode_active = !self.grid_mode_active;
                } else if self.grid_mode_active {
                    if key == Key::SemiColon {
                        backspace();
                        rdev::simulate(&EventType::ButtonPress(Left)).unwrap();
                        rdev::simulate(&EventType::ButtonRelease(Left)).unwrap();
                        self.grid_mode_active = !self.grid_mode_active;
                    } else if key == Key::LeftBracket {
                        backspace();
                        rdev::simulate(&EventType::ButtonPress(Left)).unwrap();
                        rdev::simulate(&EventType::ButtonRelease(Left)).unwrap();
                        rdev::simulate(&EventType::ButtonPress(Left)).unwrap();
                        rdev::simulate(&EventType::ButtonRelease(Left)).unwrap();
                        self.grid_mode_active = !self.grid_mode_active;
                    } else if key == Key::Quote {
                        backspace();
                        rdev::simulate(&EventType::ButtonPress(Right)).unwrap();
                        rdev::simulate(&EventType::ButtonRelease(Right)).unwrap();
                        self.grid_mode_active = !self.grid_mode_active;
                    } else if key == Key::Comma {
                        backspace();
                        rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: 1 }).unwrap();
                    } else if key == Key::Dot {
                        backspace();
                        rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: -1 }).unwrap();
                    } else if key == Key::Slash {
                        rdev::simulate(&EventType::ButtonPress(Middle)).unwrap();
                        rdev::simulate(&EventType::ButtonRelease(Middle)).unwrap();
                        backspace();
                        self.grid_mode_active = !self.grid_mode_active;
                    } else if key == Key::Backspace {} else {
                        let mut key_string = format!("{:?}", key);
                        key_string = key_string.as_str().replace("Key", "");
                        key_string = key_string.as_str().replace("Num", "");
                        key_string = key_string.to_lowercase();
                        dbg!(&key_string);
                        if key_string.len() > 1 {
                            println!("Clear buffer");
                            self.input_buffer.clear();
                        } else {
                            if self.backspace_inited { backspace() } else { self.backspace_inited = true; }

                            if self.input_buffer.len() > 3 {
                                self.input_buffer.remove(0);
                            }
                            self.input_buffer.push(key_string.chars().next().unwrap());
                            println!("Buffer: {:?}", self.input_buffer);

                            let key: String = self.input_buffer.iter().collect();
                            let point = self.find_point_by_key(&key)
                                .or_else(|| {
                                    let len = self.input_buffer.len();
                                    if len > 1 {
                                        let key: String = self.input_buffer[1..len].iter().collect();
                                        self.find_point_by_key(&key)
                                    } else { None }
                                })
                                .or_else(|| {
                                    let len = self.input_buffer.len();
                                    if len > 2 {
                                        let key: String = self.input_buffer[2..len].iter().collect();
                                        self.find_point_by_key(&key)
                                    } else { None }
                                })
                                .or_else(|| {
                                    let len = self.input_buffer.len();
                                    if len > 3 {
                                        let key: String = self.input_buffer[3..len].iter().collect();
                                        self.find_point_by_key(&key)
                                    } else { None }
                                });
                            if point.is_some() {
                                let Point { x, y, key: _ } = point.unwrap();
                                println!("Found point: {:?}", point);
                                rdev::simulate(&EventType::MouseMove { x: x.clone() as f64, y: y.clone() as f64 }).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_point_by_key(&self, key: &str) -> Option<&Point> {
        println!("Search by key: {}", key);
        self.points.iter()
            .find(|point| point.key.eq(key))
    }
}

fn backspace() {
    rdev::simulate(&EventType::KeyPress(Key::Backspace)).unwrap();
    rdev::simulate(&EventType::KeyRelease(Key::Backspace)).unwrap();
}

impl UserApp<GlfwWindow, WgpuBackend> for Application {
    fn run(&mut self, ctx: &egui::Context, glfw_backend: &mut GlfwWindow, _: &mut WgpuBackend) {
        glfw_backend.window.set_mouse_passthrough(true);
        self.draw_grid(ctx);
        self.handle_input();
    }
}

fn start_overlay(app: impl UserApp<GlfwWindow, WgpuBackend> + 'static) {
    let mut glfw_backend = new_window(Default::default(), Default::default());
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
    let image = image::load_from_memory(include_bytes!("../../assets/icon.png"))
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

pub fn init() {
    let (schan, rchan) = mpsc::channel();
    thread::spawn(move || {
        rdev::listen(move |event| {
            let Event { time: _, name: _, event_type } = &event;
            if let EventType::KeyRelease(key) = event_type {
                println!("Key pressed {:?}", key);
                schan
                    .send(event)
                    .unwrap_or_else(|e| println!("Could not send event {:?}", e));
            }
        }).expect("Could not listen");
    });

    start_overlay(Application::new(rchan));
}
