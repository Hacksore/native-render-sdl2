use objc::msg_send;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::sys::{SDL_GetVersion, SDL_GetWindowWMInfo, SDL_SysWMinfo, SDL_bool, SDL_SYSWM_TYPE};
use sdl2::video::{Window, WindowPos};
use std::time::Duration;

fn get_macos_window_handle(window: &Window) -> Option<*mut std::ffi::c_void> {
  let mut wm_info: SDL_SysWMinfo = unsafe { std::mem::zeroed() };
  unsafe {
    SDL_GetVersion(&mut wm_info.version);
    if SDL_GetWindowWMInfo(window.raw(), &mut wm_info) == SDL_bool::SDL_FALSE {
      return None;
    }
  }

  unsafe {
    match wm_info.subsystem {
      SDL_SYSWM_TYPE::SDL_SYSWM_COCOA => {
        // The exact field name might vary, so you may need to adjust this
        // It could be `wm_info.info.cocoa.window` or `wm_info.info.x11.window` or something else
        // You may need to inspect the `SDL_SysWMinfo` struct definition in your SDL2 bindings
        Some(wm_info.info.x11.window as *mut std::ffi::c_void)
      }
      _ => None,
    }
  }
}

pub fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let mut window = video_subsystem
    .window("rust-sdl2 demo: Video", 2560, 1440)
    // .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

  window.set_bordered(false);
  window.set_always_on_top(true);
  window.set_position(WindowPos::Positioned(0), WindowPos::Positioned(0));

  // give me the cocoa raw window handle
  let native_window = get_macos_window_handle(&window).unwrap();

  unsafe {
    // get windowd id
    let window_id: u64 = msg_send![native_window, windowNumber];
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
