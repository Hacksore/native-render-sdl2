use objc::msg_send;
use raw_window_handle::{AppKitWindowHandle, HasRawWindowHandle, RawWindowHandle};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowPos;
use std::time::Duration;

// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/raw-window-handle-with-wgpu/main.rs
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/src/sdl2/raw_window_handle.rs#L114-L131
// https://github.com/Rust-SDL2/rust-sdl2/pull/1275
// https://github.com/kelpsyberry/dust/blob/987ebb46f34ce81db839da0e2190d261ccec7bc2/frontend/desktop/src/ui/window.rs
// NOTE: this might be it
// https://github.com/Rust-SDL2/rust-sdl2/pull/962/files

pub fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let mut window = video_subsystem
    .window("rust-sdl2 demo: Video", 2560, 1440)
    // .position_centered()
    .opengl()
    .metal_view()
    .build()
    .map_err(|e| e.to_string())?;

  window.set_bordered(false);
  window.set_always_on_top(true);
  window.set_position(WindowPos::Positioned(0), WindowPos::Positioned(0));

  unsafe {
    let raw_window = window.raw_window_handle().unwrap();
    
    match raw_window {
      RawWindowHandle::AppKit(AppKitWindowHandle { ns_view, .. }) => {
        let ns_window = ns_view.window();
        // set the window bg to clear
        let _: () = msg_send![ns_window, setBackgroundColor:objc::runtime::NO];
      }
      _ => (),
    }
  }

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

  let mut x = 0;
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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 200));
    canvas.fill_rect(Rect::new(x, 0, 100, 100))?;
    canvas.present();

    x += 1;
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    // The rest of the game loop goes here...
  }

  Ok(())
}
