use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

use crate::{asset_loader::SceneAssets, collision_detection::Collider, movement::{Acceleration, MoveObjectBundle, Velocity}};

const VELOCITY_SCALARE : f32 = 5.0;
const ACCELERATION_SCALARE : f32 = 1.0;
const SPAWN_RANGE_X : Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z : Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS : f32 = 1.0;
const ROTATE_SPEED : f32 = 2.5;
const RADIUS : f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroids;

pub struct AsteroidsPlugin;
impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SpawnTimer {
            timer : Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)
        })
        .add_systems(Update, 
            (spawn_asteroids, rotate_asteroids, handle_asteroid_collisions)
        );
    }
}


#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer : Timer,
}

fn spawn_asteroids(
    mut commands : Commands, mut spawn_timer : ResMut<SpawnTimer>,
    time : Res<Time>, scene_assets : Res<SceneAssets>
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector = 
    || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0));


    let velocity = random_unit_vector() * VELOCITY_SCALARE;
    let acceleration = random_unit_vector() * ACCELERATION_SCALARE;

    commands.spawn((
        MoveObjectBundle {
            velocity : Velocity::new(velocity),
            collider : Collider::new(RADIUS),
            acceleration : Acceleration::new(acceleration),
            model : SceneBundle {
                scene : scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            } 
        },
        Asteroids,
    ));
}

fn rotate_asteroids(mut query : Query<&mut Transform, With<Asteroids>>, time : Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}


fn handle_asteroid_collisions(
    mut commands : Commands,
    query : Query<(Entity, &Collider), With<Asteroids>>
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}