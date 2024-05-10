use std::time::Duration;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    time::{Time, Timer, TimerMode},
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

#[derive(Component)]
pub struct Countdown {
    pub timer: Timer,
}

fn count(mut commands: Commands, mut query: Query<(&mut Countdown, Entity)>, time: Res<Time>) {
    for (mut countdown, entity) in &mut query {
        countdown.timer.tick(time.delta());
        if countdown.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
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

pub type AllowedToRun = bool;
// errors if you should
pub fn countdown_guard(query: Query<&Countdown>) -> AllowedToRun {
    for count in &query {
        if !count.timer.finished() {
            return false;
        }
    }

    true
}

fn start_countdown(mut commands: Commands) {
    commands.spawn(Countdown {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
    });
}

fn start_countdown_on_score(mut commands: Commands, mut events: EventReader<Scored>) {
    for event in events.read() {
        start_countdown(commands);
        return;
    }
}

pub struct GameManagerPlugin;
impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Scored>()
            .init_resource::<Score>()
            .add_systems(Startup, start_countdown)
            .add_systems(Update, (detect_scoring, count, start_countdown_on_score));
    }
}
