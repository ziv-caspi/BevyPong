use bevy::{
    ecs::entity::Entity,
    math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{
    game_manager::{countdown_guard, AllowedToRun, Scored},
    spritesheet_animation::{AnimationIndices, AnimationTimer},
    utils::{ball_collision, project_positions, Collision, Position, Shape, Velocity},
};
use anyhow::Result;

const INITIAL_SPEED: f32 = 6.0;
const RADIUS: f32 = 20.0;

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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let shape = Mesh::from(Circle::new(RADIUS));
    let color = ColorMaterial::from(Color::rgb(1., 0., 0.));

    let texture = asset_server.load("fireball.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(71.3, 45.6), 3, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 8 };

    commands.spawn((
        BallBundle::new(),
        // MaterialMesh2dBundle {
        //     mesh: mesh_handle.into(),
        //     material: color_handle,
        //     ..default()
        // },
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            //transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn move_ball(
    In(allowed): In<AllowedToRun>,
    mut ball: Query<(&mut Position, &Velocity), With<Ball>>,
) {
    if !allowed {
        return;
    }

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

fn adjust_sprite_flip_rotation(
    mut ball: Query<(&mut Sprite, &mut Transform, &Velocity), With<Ball>>,
) {
    _ = || -> Result<()> {
        let (mut sprite, mut transform, velocity) = ball.get_single_mut()?;
        if velocity.0.x > 0. {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }

        let flip_modifier: f32;
        if sprite.flip_x {
            flip_modifier = 1.0;
        } else {
            flip_modifier = -1.0;
        }

        let angle_modifier: f32;
        if velocity.0.y > 0.0 {
            angle_modifier = 1.;
        } else {
            angle_modifier = -1.;
        }

        let angle = flip_modifier * angle_modifier * 45.0;
        *transform = transform.with_rotation(Quat::from_rotation_z(angle.to_radians()));
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
                    adjust_sprite_flip_rotation,
                    reset_on_score,
                    countdown_guard.pipe(move_ball),
                    project_positions.after(move_ball),
                    collision.after(move_ball),
                ),
            )
            .add_event::<BallCollision>();
    }
}
