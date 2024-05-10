use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Query, ResMut, Resource},
    },
};

use crate::{
    ball::BallCollision,
    border::{self, Border},
    utils::Position,
};

#[derive(Event)]
pub enum Scored {
    Player,
    Ai,
}

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

fn detect_scoring(
    borders: Query<&Border, With<Border>>,
    mut events: EventReader<BallCollision>,
    mut events_writer: EventWriter<Scored>,
    mut score: ResMut<Score>,
) {
    for event in events.read() {
        if let Ok(border) = borders.get(event.entity) {
            match border {
                Border::Left => {
                    events_writer.send(Scored::Ai);
                    score.ai += 1;
                }
                Border::Right => {
                    events_writer.send(Scored::Player);
                    score.player += 1;
                }
                Border::Top => {}
                Border::Bottom => {}
            };
        }
    }
}

pub struct GameManagerPlugin;
impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Scored>()
            .init_resource::<Score>()
            .add_systems(Update, detect_scoring);
    }
}
