use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn hello_world() {
    eprintln!("GRAAARGH! HELLO WORLDS!!!!!!!!!!!")
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("silly".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<&mut Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut name in &mut query {
            eprintln!("Hello, {}!", name.0);
            if name.0 == "silly" {
                name.0 = "NOT SILLY!!!!!".to_string();
            }
        }
    }

}