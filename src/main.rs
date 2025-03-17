use bevy::ecs::system::StaticSystemInput;
use bevy::input::{InputPlugin, InputSystem};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_rts_camera::{RtsCameraPlugin, RtsCamera};
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use hexx::*;

mod icoglobe;
mod vec;
mod prng;
mod hsh;
mod math_trait;
mod vert_data;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RtsCameraPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .add_system(switch_camera_mode)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct CameraMode {
    mode: Mode,
}

#[derive(PartialEq)]
enum Mode {
    Rts,
    PanOrbit,
}

fn setup(mut commands: Commands) {
    // Setup RTS Camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
                    RtsCamera {
                        ..default()
                    },
                    MainCamera,
    ));

    // Initialize Camera Mode
    commands.insert_resource(CameraMode { mode: Mode::Rts });

    // Spawn a basic hex grid for testing
    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        origin: Vec2::ZERO,
        scale: Default::default(),
    };
    let hexes = Hex::range(Hex::ZERO, 5);
    for hex in hexes {
        let pos = layout.hex_to_world_pos(hex);
        commands.spawn(PbrBundle {
            mesh: meshes::shape::Cylinder::new(0.1, 0.5).mesh(),
            material: materials::Color::rgb(0.8, 0.7, 0.6).into(),
            transform: Transform::from_xyz(pos.x, 0.0, pos.y),
            ..default()
        });
    }

    // Spawn a light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn switch_camera_mode(
    mut camera_mode: ResMut<CameraMode>,
    keyboard_input: Res<StaticSystemInput<KeyboardInput>>,
    mut rts_cameras: Query<&mut RtsCamera, With<MainCamera>>,
    mut pan_orbit_cameras: Query<&mut PanOrbitCamera, With<MainCamera>>,
) {
    if keyboard_input.(KeyCode::Tab) {
        if camera_mode.mode == Mode::Rts {
            camera_mode.mode = Mode::PanOrbit;
            if let Ok(mut rts_camera) = rts_cameras.get_single_mut() {
                rts_camera.enabled = false;
            }
            if let Ok(mut pan_orbit_camera) = pan_orbit_cameras.get_single_mut() {
                pan_orbit_camera.enabled = true;
            }
        } else {
            camera_mode.mode = Mode::Rts;
            if let Ok(mut rts_camera) = rts_cameras.get_single_mut() {
                rts_camera.enabled = true;
            }
            if let Ok(mut pan_orbit_camera) = pan_orbit_cameras.get_single_mut() {
                pan_orbit_camera.enabled = false;
            }
        }
    }
}
