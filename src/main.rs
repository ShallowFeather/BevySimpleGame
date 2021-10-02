mod arrow;
mod characters;
mod score;

use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;


fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "owo!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1., 0.4, 0.4)))
        .init_resource::<score::ScoreBoard>()
        .add_startup_stage("game_setup", SystemStage::single(characters::spawn_main.system()).with_system(score::score_ui.system()))
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(characters::CharacterPlugin)
        //.add_plugin(score::UI)
        .add_plugin(arrow::ArrowsPlugin)
        .add_system(score::update_score_text.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .spawn_bundle(UiCameraBundle::default());
}
