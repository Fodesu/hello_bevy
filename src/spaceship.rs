use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::Acceleration;
use crate::movement::Velocity;
use crate::movement::MoveObjectBundle;




const STARTING_TRANSLATION : Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED : f32 = 25.0;
const SPACESHIP_ROTATION_SPEED : f32 = 2.5;
const SPACESHIP_ROLL_SPEED : f32 = 2.5;
const SPACESHIP_RADIUS : f32 = 5.0;
const MISSILE_SPEED : f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR : f32 = 7.5; 
const MISSILE_RADIUS : f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

#[derive(Component, Debug)]
pub struct SpaceshipMissile; 

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PostStartup, spawn_spaceship)
        .add_systems(Update, spaceship_movement_control)
        .add_systems(Update, spaceship_weapon_controls);
    }
}

fn spawn_spaceship(mut commands : Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MoveObjectBundle {
            velocity : Velocity::new(Vec3::ZERO),
            collider : Collider::new(SPACESHIP_RADIUS),
            acceleration : Acceleration::new(Vec3::ZERO),
            model : SceneBundle {
                scene : scene_assets.spaceship.clone(),
                transform : Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_control(
    mut query : Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input : Res<Input<KeyCode>>,
    time : Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        rotation = - SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }


    if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = - SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    velocity.value = -transform.forward() * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}

fn spaceship_weapon_controls(
    mut commands : Commands,
    query : Query<&Transform, With<Spaceship>>,
    keyboard_input : Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn(
            (MoveObjectBundle {
                velocity : Velocity::new(-transform.forward() * MISSILE_SPEED),
                collider : Collider::new(MISSILE_RADIUS),
                acceleration : Acceleration::new(Vec3::ZERO),
                model : SceneBundle {
                    scene : scene_assets.missiles.clone(),
                    transform : Transform::from_translation(transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}