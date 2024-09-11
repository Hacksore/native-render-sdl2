//! Shows how to display a window in transparent mode.
//!
//! This feature works as expected depending on the platform. Please check the
//! [documentation](https://docs.rs/bevy/latest/bevy/prelude/struct.Window.html#structfield.transparent)
//! for more details.

#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;
use bevy::{
  prelude::*,
  sprite::{MaterialMesh2dBundle, Mesh2dHandle},
  window::{Cursor, WindowLevel},
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
        transparent: true, // Disabling window decorations to make it feel more like a widget than a window
        window_level: WindowLevel::AlwaysOnTop,
        cursor: Cursor {
          // Allow inputs to pass through to apps behind this app.
          hit_test: false,
          ..default()
        },
        decorations: false,
        #[cfg(target_os = "macos")]
        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
        ..default()
      }),
      ..default()
    }))
    // ClearColor must have 0 alpha, otherwise some color will bleed through
    .insert_resource(ClearColor(Color::NONE))
    .add_systems(Startup, setup)
    .add_systems(Update, on_render)
    .run();
}
const X_EXTENT: f32 = 900.;

fn on_render() {
  // render 
  println!("render");
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(Camera2dBundle::default());

  let square = Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0)));

  // Distribute colors evenly across the rainbow.
  let color = Color::hsl(0.0, 0.95, 0.7);

  commands.spawn(MaterialMesh2dBundle {
    mesh: square,
    material: materials.add(color),
    transform: Transform::from_xyz(
      0.0,
      0.0,
      0.0,
    ),
    ..default()
  });

  #[cfg(not(target_arch = "wasm32"))]
  commands.spawn(
    TextBundle::from_section("Bevy just works", TextStyle::default()).with_style(
      Style {
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        ..default()
      },
    ),
  );
}
