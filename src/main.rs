use notan::draw::*;
use notan::prelude::*;
use raw_window_handle::RawWindowHandle;

#[notan_main]
fn main() -> Result<(), String> {
  let win = WindowConfig::default()
    .set_transparent(true)
    .set_mouse_passthrough(true)
    .set_always_on_top(true)
    .set_decorations(false);

  notan::init_with(setup)
    .add_config(DrawConfig)
    .add_config(win)
    .draw(draw)
    .build()
}

fn setup(app: &mut App, gfx: &mut Graphics) {
  println!("app {:?}", app.timer.delta());

  let window  = app.window();
  // let backend = &app.backend;

  println!("{}", window.is_always_on_top());
}

fn draw(app: &mut App, gfx: &mut Graphics) {
  let window  = app.window();
  println!("{}", window.is_always_on_top());
  let mut draw = gfx.create_draw();
  draw.clear(Color::TRANSPARENT);
  draw.rect((100.0, 100.0), (600.0, 400.0));
  gfx.render(&draw);
}
