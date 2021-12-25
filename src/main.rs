fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_context = sdl_context.video().unwrap();
    let window = video_context.window("ReBLCK", 1280, 720).build().unwrap();
}

