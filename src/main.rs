use cocoa::appkit::{NSColor, NSWindow};
use cocoa::base::{nil, NO};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use raw_window_handle::{AppKitWindowHandle, HasRawWindowHandle, RawWindowHandle};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;
use std::time::Duration;

pub enum NSWindowSharingType {
  NSWindowSharingNone = 0,
  NSWindowSharingReadOnly = 1,
  NSWindowSharingReadWrite = 2,
}

pub fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let mut window = video_subsystem
    .window("rust-sdl2 demo: Video", 600, 600)
    .position_centered()
    .allow_highdpi()
    .opengl()
    .metal_view()
    .build()
    .map_err(|e| e.to_string())?;

  window.set_bordered(false);
  window.set_always_on_top(true);

  unsafe {
    let raw_window = window.raw_window_handle().unwrap();

    match raw_window {
      RawWindowHandle::AppKit(AppKitWindowHandle { ns_view, .. }) => {
        let ns_view: *mut Object = ns_view.as_ptr().cast();
        let ns_window: *mut Object = msg_send![ns_view, window];
        ns_window.setBackgroundColor_(NSColor::clearColor(nil));
        ns_window.setOpaque_(NO);
        ns_window.setHasShadow_(NO);

        // just to prove that this is working by letting you not see CMD+tab
        ns_window.setLevel_(1001);

        // set the sharing type
        // let _: () = msg_send![ns_window, setSharingType:NSWindowSharingType::NSWindowSharingNone];

        ns_view.setBackgroundColor_(NSColor::clearColor(nil));
        ns_view.setOpaque_(NO);

        println!("window: {:?}", ns_window);
      }

      _ => (),
    }
  }

  let mut canvas = window
    .into_canvas()
    .accelerated()
    .target_texture()
    .build()
    .map_err(|e| e.to_string())?;

  let mut x = 0;
  let mut event_pump = sdl_context.event_pump()?;
  canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

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

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 200));
    canvas.fill_rect(Rect::new(x, 0, 100, 100))?;
    canvas.present();
    x += 1;

    if x > 100 {
      x = 0;
    }

    thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    // The rest of the game loop goes here...
  }

  Ok(())
}
