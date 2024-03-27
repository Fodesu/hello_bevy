mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDeterctionPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CollisionDeterctionPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(AsteroidsPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
