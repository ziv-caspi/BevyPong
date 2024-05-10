use bevy::prelude::*;

use crate::utils::{Position, Shape};

const INFINITE: f32 = 100000.;

#[derive(Component)]
pub enum Border {
    Left,
    Right,
    Top,
    Bottom,
}

impl Border {
    fn get_position(&self, width: f32, height: f32) -> Position {
        match self {
            Border::Left => Position(Vec2::new(-width, 0.)),
            Border::Right => Position(Vec2::new(width, 0.)),
            Border::Top => Position(Vec2::new(0., height)),
            Border::Bottom => Position(Vec2::new(0., -height)),
        }
    }
}

#[derive(Bundle)]
struct BorderBundle {
    border: Border,
    position: Position,
    shape: Shape,
}

fn spawn(mut commands: Commands, window: Query<&Window>) {
    let win = window.get_single().unwrap();
    let height = win.resolution.height() / 2.;
    let width = win.resolution.width() / 2.;
    println!("screen dimensions, height: {}, width: {}", height, width);

    let vertical = Shape::Rectangle {
        width: 20.,
        height: INFINITE,
    };

    let horizontal = Shape::Rectangle {
        width: INFINITE,
        height: 20.,
    };

    commands.spawn(BorderBundle {
        border: Border::Left,
        position: Border::Left.get_position(width, height),
        shape: vertical.clone(),
    });

    commands.spawn(BorderBundle {
        border: Border::Right,
        position: Border::Right.get_position(width, height),
        shape: vertical.clone(),
    });

    commands.spawn(BorderBundle {
        border: Border::Top,
        position: Border::Top.get_position(width, height),
        shape: horizontal.clone(),
    });

    commands.spawn(BorderBundle {
        border: Border::Bottom,
        position: Border::Bottom.get_position(width, height),
        shape: horizontal.clone(),
    });
}

fn adjust_border_position(
    mut borders: Query<(&mut Position, &Border), With<Border>>,
    window: Query<&Window>,
) {
    let win = window.get_single().unwrap();
    let height = win.height() / 2.;
    let width = win.width() / 2.;

    for (mut position, border) in &mut borders {
        position.0 = border.get_position(width, height).0;
    }
}

pub struct BordersPlugin;
impl Plugin for BordersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, adjust_border_position);
    }
}
