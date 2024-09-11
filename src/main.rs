//! Shows how to display a window in transparent mode.
//!
//! This feature works as expected depending on the platform. Please check the
//! [documentation](https://docs.rs/bevy/latest/bevy/prelude/struct.Window.html#structfield.transparent)
//! for more details.

#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;
use bevy::{
  prelude::*,
  sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::{Cursor, WindowLevel},
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
    .run();
}
const X_EXTENT: f32 = 900.;
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(Camera2dBundle::default());

  let shapes = [
    Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
    Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
    Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
    Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
    Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
    Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
    Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
    Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
    Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
    Mesh2dHandle(meshes.add(Triangle2d::new(
      Vec2::Y * 50.0,
      Vec2::new(-50.0, -50.0),
      Vec2::new(50.0, -50.0),
    ))),
  ];
  let num_shapes = shapes.len();

  for (i, shape) in shapes.into_iter().enumerate() {
    // Distribute colors evenly across the rainbow.
    let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

    commands.spawn(MaterialMesh2dBundle {
      mesh: shape,
      material: materials.add(color),
      transform: Transform::from_xyz(
        // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
        -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
        0.0,
        0.0,
      ),
      ..default()
    });
  }

  #[cfg(not(target_arch = "wasm32"))]
  commands.spawn(
    TextBundle::from_section("Press space to toggle wireframes", TextStyle::default()).with_style(
      Style {
        position_type: PositionType::Absolute,
        top: Val::Px(12.0),
        left: Val::Px(12.0),
        ..default()
      },
    ),
  );
}
