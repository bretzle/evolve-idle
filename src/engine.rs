use crate::{clockwork::Clockwork, Game};
use glow::HasContext;
use glutin::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, PossiblyCurrent, WindowedContext,
};
use imgui::FontSource;
use imgui_glow_renderer::AutoRenderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::{Duration, Instant};

pub struct Engine {
    event_loop: EventLoop<()>,
    window: WindowedContext<PossiblyCurrent>,
    platform: WinitPlatform,
    pub imgui: imgui::Context,
    renderer: AutoRenderer,
    last_frame: Instant,
    pub clockwork: Clockwork<Game>,
    game: Game,
}

impl Engine {
    pub fn new(title: &str, [width, height]: [i32; 2]) -> Self {
        let event_loop = EventLoop::new();
        let window = unsafe {
            ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(
                    WindowBuilder::new()
                        .with_title(title)
                        .with_inner_size(LogicalSize::new(width, height)),
                    &event_loop,
                )
                .expect("could not create window")
                .make_current()
                .expect("could not make window context current")
        };

        let mut imgui = imgui::Context::create();
        let mut platform = WinitPlatform::init(&mut imgui);

        imgui.set_ini_filename(None);
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: (13.0 * platform.hidpi_factor()) as f32,
                ..Default::default()
            }),
        }]);
        imgui.io_mut().font_global_scale = (1.0 / platform.hidpi_factor()) as f32;

        platform.attach_window(imgui.io_mut(), window.window(), HiDpiMode::Rounded);

        let gl = unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) };

        let mut engine = Self {
            event_loop,
            window,
            platform,
            renderer: AutoRenderer::initialize(gl, &mut imgui).unwrap(),
            imgui,
            last_frame: Instant::now(),
            clockwork: Clockwork::new(),
            game: gemstone::mem::zero(),
        };
        engine.game = Game::new(&mut engine);
        engine
    }

    pub fn run(self) -> ! {
        let Self {
            event_loop,
            window,
            mut platform,
            mut imgui,
            mut renderer,
            mut last_frame,
            mut clockwork,
            mut game,
        } = self;

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                Event::MainEventsCleared => {
                    platform.prepare_frame(imgui.io_mut(), window.window()).unwrap();
                    window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    // The renderer assumes you'll be clearing the buffer yourself
                    unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

                    let ui = imgui.frame();
                    game.update(ui);

                    platform.prepare_render(ui, window.window());
                    let draw_data = imgui.render();

                    // This is the only extra render step to add
                    renderer.render(draw_data).expect("error rendering imgui");

                    window.swap_buffers().unwrap();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    game.on_exit();
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                event => {
                    platform.handle_event(imgui.io_mut(), window.window(), &event);
                }
            }

            clockwork.run_pending(&mut game);

            *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(1000 / 60));
        });
    }
}
