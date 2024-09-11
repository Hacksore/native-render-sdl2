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

// A component to tag our square
#[derive(Component)]
struct Square;

// create an on render that moves the sqaure we made in setup
fn on_render(time: Res<Time>, mut query: Query<&mut Transform, With<Square>>) {
  for mut transform in query.iter_mut() {
    transform.translation.x += time.delta_seconds() * 100.0; // adjust speed as needed
  }
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

  commands
    .spawn(MaterialMesh2dBundle {
      mesh: square,
      material: materials.add(color),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    })
    .insert(Square);
}
