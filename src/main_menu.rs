use crate::{despawn_entities, GameState};
use bevy::color::palettes::css::{GREEN, PINK, RED, YELLOW};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(OnExit(GameState::Menu), despawn_entities::<MenuEntities>)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)));
    }
}

#[derive(Component)]
enum MenuButtonAction {
    PlayGame,
    Dont,
}

// Tag component for all main menu entities
#[derive(Component)]
struct MenuEntities;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MenuEntities));
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }, 
        MenuEntities
    ))
        .with_children(|main_menu| {
            main_menu.spawn(TextBundle::from_section(
                "cube simulator 3D",
                TextStyle {
                    font: asset_server.load("C:\\Windows\\Fonts\\COMIC.TTF"),
                    font_size: 200.0,
                    color: Color::from(RED),
                },
            ));

            // Main menu content

            // Common button style and text style for buttons
            let button_bundle = ButtonBundle {
                style: Style {
                    width: Val::Vw(50.0),
                    height: Val::Vw(20.0),
                    border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::from(YELLOW)),
                background_color: BackgroundColor(Color::from(GREEN)),
                ..default()
            };
            let button_text_style = TextStyle {
                font: asset_server.load("C:\\Windows\\Fonts\\COMIC.TTF"),
                font_size: 100.0,
                color: Color::from(RED),
            };

            main_menu.spawn((button_bundle.clone(), MenuButtonAction::PlayGame)).with_children(|play_game_button| {
                play_game_button.spawn(TextBundle::from_section("play the game", button_text_style.clone()));
            });
            main_menu.spawn((button_bundle.clone(), MenuButtonAction::Dont)).with_children(|exit_game_button| {
                exit_game_button.spawn(TextBundle::from_section("don't", button_text_style));
            });
        });
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut colour, action) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                match action {
                    MenuButtonAction::PlayGame => game_state.set(GameState::Playing),
                    MenuButtonAction::Dont => { exit.send(AppExit::Success); }
                }
            },
            Interaction::Hovered => colour.0 = Color::from(PINK),
            Interaction::None => colour.0 = Color::from(GREEN),
        }
    }
}