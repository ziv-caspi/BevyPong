use crate::{
    ball::Ball,
    utils::{project_positions, Position, Shape, Velocity},
};
use anyhow::Result;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const WIDTH: f32 = 10.;
const HEIGHT: f32 = 100.;
const SPEED: f32 = 5.;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    position: Position,
    velocity: Velocity,
    shape: Shape,
}

fn spawn(
    mut commmands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window_width = window.get_single().unwrap().resolution.width();
    let padding = 50.;
    let right_paddle_x = window_width / 2. - padding;
    let left_paddle_x = -window_width / 2. + padding;

    let shape = Mesh::from(Rectangle::new(WIDTH, HEIGHT));
    let player_color = ColorMaterial::from(Color::GREEN);
    let ai_color = ColorMaterial::from(Color::BLACK);

    let mesh_handle = meshes.add(shape);
    let player_color_handle = materials.add(player_color);
    let ai_color_handle = materials.add(ai_color);

    commmands.spawn((
        Player,
        PaddleBundle {
            paddle: Paddle,
            shape: Shape::Rectangle {
                width: WIDTH,
                height: HEIGHT,
            },
            position: Position(Vec2 {
                x: right_paddle_x,
                y: -25.,
            }),
            velocity: Velocity(Vec2::new(0., 0.)),
        },
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: player_color_handle.clone(),
            ..Default::default()
        },
    ));

    commmands.spawn((
        Ai,
        PaddleBundle {
            paddle: Paddle,
            shape: Shape::Rectangle {
                width: WIDTH,
                height: HEIGHT,
            },
            position: Position(Vec2 {
                x: left_paddle_x,
                y: -25.,
            }),
            velocity: Velocity(Vec2::new(0., 0.)),
        },
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: ai_color_handle.clone(),
            ..Default::default()
        },
    ));
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, (With<Paddle>, With<Player>)>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -SPEED;
        } else if input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = SPEED;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn ai_paddle(
    mut paddle: Query<(&mut Velocity, &Position), (With<Paddle>, With<Ai>)>,
    ball: Query<&Position, With<Ball>>,
) {
    let _ = || -> Result<()> {
        let (mut ai_velocity, ai_position) = paddle.get_single_mut()?;
        let ball_position = ball.get_single()?;

        let diff = ai_position.0 - ball_position.0;
        let y_diff = diff.y;
        if y_diff > 0. {
            ai_velocity.0.y = -SPEED;
        } else if y_diff < 0. {
            ai_velocity.0.y = SPEED;
        } else {
            ai_velocity.0.y = 0.;
        }

        Ok(())
    }();
}

fn move_paddles(mut paddles: Query<(&mut Position, &Velocity), With<Paddle>>) {
    for (mut position, velocity) in &mut paddles {
        position.0 += velocity.0;
    }
}

pub struct PaddlesPlugin;
impl Plugin for PaddlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(
            Update,
            (ai_paddle, handle_input, move_paddles, project_positions).chain(),
        );
    }
}
