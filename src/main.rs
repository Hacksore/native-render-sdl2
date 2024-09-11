use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::WindowPos;
use std::time::Duration;

pub fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let mut window = video_subsystem
    .window("rust-sdl2 demo: Video", 2560, 1440)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

  window.set_bordered(false);
  window.set_always_on_top(true);
  window.set_position(WindowPos::Positioned(0), WindowPos::Positioned(0));

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

  canvas.set_draw_color(Color::RGB(255, 0, 0));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump()?;

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }

    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    // The rest of the game loop goes here...
  }

  Ok(())
}
