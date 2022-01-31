use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let window_size_x = 1280.0;
    let window_size_y = 720.0;

    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title(format!("RΞBLCK - Development - v{}", VERSION))
        .with_inner_size(glutin::dpi::LogicalSize::new(1280.0, 720.0))
        .with_resizable(false);
    let display = Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_win = display.gl_window();
        let win = gl_win.window();

        platform.attach_window(imgui.io_mut(), &win, HiDpiMode::Default);
    }

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    'running: loop {
        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            },
            Event::MainEventsCleared => {
                let gl_win = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), &gl_win.window())
                    .expect("Failed to prepare frame");
                gl_win.window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                let mut ui = imgui.frame();

                let mut run = true;

                /* Ui */

                // Ui code will go here

                /******/

                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_win = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
                platform.prepare_render(&ui, gl_win.window());
                let draw_data = ui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_win = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_win.window(), &event);
            }
        });
    }
}