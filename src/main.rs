use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rect;
use speedy2d::window::{WindowCreationOptions, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
  let window = Window::new_with_options(
    "Title",
    WindowCreationOptions::new_windowed(
      speedy2d::window::WindowSize::PhysicalPixels(Vector2::new(500, 500)),
      None,
    )
    .with_decorations(false)
    .with_always_on_top(true)
    .with_transparent(true),
  )
  .unwrap();
  window.run_loop(MyWindowHandler {});
}

struct MyWindowHandler {}

impl WindowHandler for MyWindowHandler {
  fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
    graphics.clear_screen(Color::from_rgba(0.0, 0.0, 0.0, 0.0));
    let start = Vector2::new(0.0, 0.0);
    let end = Vector2::new(100.0, 100.0);
    graphics.draw_rectangle(Rect::new(start, end), Color::BLUE);
    helper.request_redraw();
  }
}
