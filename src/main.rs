#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::cube::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::window::WindowMode;

mod hello_plugin;
mod cube;
mod main_menu;

#[derive(Clone, Eq, Debug, Hash, States, Default, PartialEq)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "cube simulator 3D".into(),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Startup, funny_doom_music)
        .init_state::<GameState>()
        .run();
}



fn funny_doom_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("doom.ogg"),
        settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.1)),
    });
}

fn despawn_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
