use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    prelude::default,
    render::color::Color,
    text::{JustifyText, Text, TextSection, TextStyle},
    ui::{node_bundles::TextBundle, Style, Val},
};

use crate::game_manager::Score;

#[derive(Component)]
struct ScoreText;

fn spawn_score(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "X",
                TextStyle {
                    font_size: 60.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::new(
                ":",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "X",
                TextStyle {
                    font_size: 60.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            align_self: bevy::ui::AlignSelf::Center,
            justify_self: bevy::ui::JustifySelf::Center,
            ..default()
        }),
        ScoreText,
    ));
}

fn udpate_score(mut text: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    let mut text_value = text.single_mut();
    text_value.sections[0].value = format!("{}", score.ai);
    text_value.sections[2].value = format!("{}", score.player);
}

pub struct GameTextPlugin;
impl Plugin for GameTextPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_score)
            .add_systems(Update, udpate_score);
    }
}
