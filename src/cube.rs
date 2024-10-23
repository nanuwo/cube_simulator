use crate::{ despawn_entities, GameState };
use bevy::app::{ App, Plugin };
use bevy::color::palettes::css::{ GREEN, PINK, RED };
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{ CursorGrabMode, PrimaryWindow };

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup, hide_cursor))
            .add_systems(OnExit(GameState::Playing), (
                despawn_entities::<GameEntities>,
                show_cursor,
            ))
            .add_systems(Update, keyboard_input.run_if(in_state(GameState::Playing)))
            .add_systems(Update, mouse_input.run_if(in_state(GameState::Playing)))
            .add_systems(PostUpdate, camera_follow_cube.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct TheCube {
    speed: f32,
}

#[derive(Component)]
struct ProCamera;

// Tag component for all game entities
#[derive(Component)]
struct GameEntities;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
                material: materials.add(Color::from(RED)),
                ..default()
            },
            TheCube { speed: 0.1 },
            GameEntities,
        ))
        .with_children(|cube| {
            cube.spawn(PointLightBundle {
                transform: Transform::from_xyz(2.0, 4.0, 2.0),
                ..default()
            });
            cube.spawn(PointLightBundle {
                transform: Transform::from_xyz(2.0, -4.0, 2.0),
                ..default()
            });
            
        });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5000.0))),
            material: materials.add(Color::from(GREEN)),
            ..default()
        },
        GameEntities,
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::NEG_Y, Vec2::splat(5000.0))),
            material: materials.add(Color::from(PINK)),
            ..default()
        },
        GameEntities,
    ));
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Dir3::Y),
            ..default()
        },
        ProCamera,
        GameEntities,
    ));
}

fn mouse_input(
    mut evr_motion: EventReader<MouseMotion>,
    cube_query: Query<&Transform, (With<TheCube>, Without<ProCamera>)>,
    mut camera_query: Query<&mut Transform, (With<ProCamera>, Without<TheCube>)>
) {
    let cube = cube_query.single();
    let mut camera = camera_query.single_mut();
    let motion: Vec2 = evr_motion
        .read()
        .map(|ev| ev.delta)
        .sum();

    let rot = Quat::from_euler(EulerRot::YXZ, -motion.x / 50.0, motion.y / 50.0, 0.0);
    camera.translate_around(cube.translation, rot);
}

fn keyboard_input(
    mut cube_query: Query<(&mut Transform, &mut TheCube)>,
    mut camera_query: Query<&mut Transform, (With<ProCamera>, Without<TheCube>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    let (mut cube_transform, mut cube) = cube_query.single_mut();

    cube.speed = if keys.just_pressed(KeyCode::Digit1) {
        0.1
    } else if keys.just_pressed(KeyCode::Digit2) {
        0.2
    } else if keys.just_pressed(KeyCode::Digit3) {
        0.3
    } else if keys.just_pressed(KeyCode::Digit4) {
        0.4
    } else if keys.just_pressed(KeyCode::Digit5) {
        0.5
    } else if keys.just_pressed(KeyCode::Digit6) {
        0.6
    } else if keys.just_pressed(KeyCode::Digit7) {
        0.7
    } else if keys.just_pressed(KeyCode::Digit8) {
        0.8
    } else if keys.just_pressed(KeyCode::Digit9) {
        0.9
    } else if keys.just_pressed(KeyCode::Digit0) {
        10.0
    } else {
        cube.speed
    };

    if keys.just_pressed(KeyCode::KeyR) {
        camera_query.single_mut().translation = cube_transform.translation + Vec3::new(
            1.5, 
            if fastrand::bool() {0.5} else {-0.5}, 
            4.2
        )
    }

    if keys.pressed(KeyCode::KeyW) {
        cube_transform.translation.z -= cube.speed;
    }
    if keys.pressed(KeyCode::KeyS) {
        cube_transform.translation.z += cube.speed;
    }
    if keys.pressed(KeyCode::KeyA) {
        cube_transform.translation.x -= cube.speed;
    }
    if keys.pressed(KeyCode::KeyD) {
        cube_transform.translation.x += cube.speed;
    }

    if keys.pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}

fn camera_follow_cube(
    target: Query<&Transform, With<TheCube>>,
    mut camera: Query<&mut Transform, (With<ProCamera>, Without<TheCube>)>
) {
    let cube = target.single();
    camera.single_mut().look_at(cube.translation, cube.translation);
}

fn hide_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window.single_mut();
    primary_window.cursor.visible = false;
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn show_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window.single_mut();
    primary_window.cursor.visible = true;
    primary_window.cursor.grab_mode = CursorGrabMode::None;
}
