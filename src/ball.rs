use bevy::{
    ecs::entity::Entity,
    math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{
    game_manager::Scored,
    utils::{ball_collision, project_positions, Collision, Position, Shape, Velocity},
};
use anyhow::Result;

const INITIAL_SPEED: f32 = 6.0;
const RADIUS: f32 = 7.0;

#[derive(Component)]
pub struct Ball;

#[derive(Event)]
pub struct BallCollision {
    pub collision: Collision,
    pub entity: Entity,
}

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
    velocity: Velocity,
    shape: Shape,
}

impl BallBundle {
    fn new() -> Self {
        BallBundle {
            ball: Ball,
            shape: Shape::Circle { radius: RADIUS },
            velocity: Velocity(Vec2::new(INITIAL_SPEED, INITIAL_SPEED)),
            position: Position(Vec2::new(0., 0.)),
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh::from(Circle::new(RADIUS));
    let color = ColorMaterial::from(Color::rgb(1., 0., 0.));

    let mesh_handle = meshes.add(shape);
    let color_handle = materials.add(color);

    commands.spawn((
        BallBundle::new(),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: color_handle,
            ..default()
        },
    ));
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0
    }
}

fn collision(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    world: Query<(&Position, &Shape, Entity), Without<Ball>>,
    mut events: EventWriter<BallCollision>,
) {
    if let Ok((mut ball_velocity, ball_position, Shape::Circle { radius })) = ball.get_single_mut()
    {
        for (position, shape, entity) in &world {
            if let Shape::Rectangle { width, height } = shape {
                if let Some(collision) = ball_collision(
                    BoundingCircle::new(ball_position.0, radius.clone()),
                    Aabb2d::new(position.0, Vec2::new(width.clone(), height.clone()) / 2.0),
                ) {
                    events.send(BallCollision { collision, entity });
                    match collision {
                        crate::utils::Collision::Left => ball_velocity.0.x *= -1.,
                        crate::utils::Collision::Right => ball_velocity.0.x *= -1.,
                        crate::utils::Collision::Top => ball_velocity.0.y *= -1.,
                        crate::utils::Collision::Bottom => ball_velocity.0.y *= -1.,
                    }
                }
            }
        }
    }
}

fn reset_on_score(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    _ = || -> Result<()> {
        let (mut position, mut velocity) = ball.get_single_mut()?;
        for event in events.read() {
            position.0 = Vec2::new(0., 0.);
            match event {
                Scored::Player => velocity.0.x *= 1.,
                Scored::Ai => velocity.0.x *= -1.,
            };
        }
        Ok(())
    }();
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(
                Update,
                (
                    reset_on_score,
                    move_ball,
                    project_positions.after(move_ball),
                    collision.after(move_ball),
                ),
            )
            .add_event::<BallCollision>();
    }
}
