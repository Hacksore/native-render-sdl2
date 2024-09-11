use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

fn draw_square(ui: &mut egui::Ui) {
  // Set the size and position of the square
  let size = Vec2::new(100.0, 100.0); // width and height
  let top_left = Pos2::new(50.0, 50.0); // position from the top-left corner

  // Define the rectangle for the square
  let rect = Rect::from_min_size(top_left, size);

  // Get the painter for the current UI area
  let painter = ui.painter();

  // Draw the square
  painter.rect(
    rect,
    0.0,                              // Rounding radius (0 means no rounding)
    Color32::from_rgb(200, 100, 100), // Fill color
    Stroke::new(2.0, Color32::BLACK), // Stroke (border width and color)
  );
}

fn main() -> Result<(), eframe::Error> {
  eframe::run_native(
    "Square Example",
    eframe::NativeOptions::default(),
    Box::new(|_cc| Ok(Box::new(SquareApp))),
  )
}

struct SquareApp;

impl eframe::App for SquareApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      draw_square(ui); // Call the function to draw the square
    });
  }
}
