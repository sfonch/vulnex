use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::{Display, DisplayApiPreference},
    prelude::{GlDisplay, NotCurrentGlContext, GlSurface},
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};
use std::{
    num::NonZeroU32,
    sync::Arc
};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

pub struct App {
    windowcfg: WindowConfig,
    window: Option<tao::window::Window>,

    context: Option<PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,

    three_d_context: Option<Arc<three_d::Context>>
}

pub struct WindowConfig {
    pub title: String,
}

impl App {
    // DEV
    fn ctx(&self) -> &Arc<three_d::Context> {
        self.three_d_context.as_ref().unwrap()
    }

    // APP CREATION
    pub fn new() -> App {
        App {
            windowcfg: WindowConfig { 
                title: "vulnex".to_string()
             },
            window: None,

            context: None,
            surface: None,

            three_d_context: None
        }
    }

    // WINDOW
    pub fn title(mut self, title: String) -> Self {
        self.windowcfg.title = title;
        self
    }

    pub fn new_title(&mut self, title: String) {
        self.windowcfg.title = title.clone();
        self.window
            .as_ref()
            .unwrap()
            .set_title(&title);
    }

    // GRAPHIC
    pub fn clear(&mut self, r: f32, g: f32, b: f32) {
        let size = self.window.as_ref().unwrap().inner_size();
    
        three_d::RenderTarget::screen(self.ctx(), size.width, size.height)
            .clear(three_d::ClearState::color(r, g, b, 1.0));
    }

    // RUN
    pub fn run<F>(mut self, mut game: F)
    where
        F: FnMut(&mut App) + 'static,
    {
        let event_loop = EventLoop::new();

        self.window = Some(WindowBuilder::new()
            .with_title(&self.windowcfg.title)
            .build(&event_loop).unwrap());

        let handle = self
            .window
            .as_ref()
            .unwrap()
            .window_handle()
            .unwrap();

        let raw = handle.as_raw();

        let display = unsafe { 
            Display::new(self.window
                .as_ref()
                .unwrap()
                .display_handle()
                .unwrap()
                .as_raw(),
                DisplayApiPreference::Wgl(Some(raw))
            ).unwrap()
         };

        let template = ConfigTemplateBuilder::new().build();
        let config = unsafe {
            display
                .find_configs(template)
                .unwrap()
                .next()
                .unwrap()
        };

        let context_attr = ContextAttributesBuilder::new().build(Some(raw));

        let not_current_context = unsafe {
            display
                .create_context(&config, &context_attr)
                .unwrap()
        };

        let surface_attr = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw,
            NonZeroU32::new(800).unwrap(),
            NonZeroU32::new(600).unwrap());

        self.surface = Some(unsafe {
            display
                .create_window_surface(&config, &surface_attr)
                .unwrap()
        });

        self.context = Some(not_current_context.make_current(self.surface.as_ref().unwrap()).unwrap());

        let glow_context = unsafe {
            glow::Context::from_loader_function(|s| {
                display
                    .get_proc_address(&std::ffi::CString::new(s).unwrap()) as *const _
             })
        };

        self.three_d_context = Some(Arc::new(
            three_d::Context::from_gl_context(Arc::new(glow_context)).unwrap(),
        ));

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    game(&mut self);
                    self.window.as_ref().unwrap().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    self.surface
                        .as_ref()
                        .unwrap()
                        .swap_buffers(self.context.as_ref().unwrap())
                        .unwrap();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                   *control_flow = ControlFlow::Exit; 
                }
                _ => ()
            }
        });
    }
    
}