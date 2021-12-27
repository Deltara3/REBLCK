use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::gfx::framerate::FPSManager;

use std::process::exit;

// TODO use this.
mod project;

const FPS: u32 = 60;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("RΞBLCK", 1280, 720).position_centered().build().unwrap();
    let mut fps_manager = FPSManager::new();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(50, 50, 50));
    fps_manager.set_framerate(FPS);

    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => {}
            }
        }
        println!("{}", fps_manager.get_frame_count());
        // draw
        canvas.clear(); // todo change this
        canvas.present();
        // fps
    }
    // handle shutdown
    exit(0);
}

