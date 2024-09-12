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
    .add_config(win)
    .add_config(DrawConfig)
    .draw(draw)
    .build()
}

fn setup(app: &mut App, gfx: &mut Graphics) {
  let win = app.window();

  let raw_handle = win.raw_window_handle();
}

fn draw(gfx: &mut Graphics) {
  let mut draw = gfx.create_draw();
  draw.clear(Color::TRANSPARENT);
  draw.rect((100.0, 100.0), (600.0, 400.0));
  gfx.render(&draw);
}
