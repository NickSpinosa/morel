use bevy::{prelude::*, DefaultPlugins};
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use config::ConfigPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod camera;
mod config;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            WorldPlugin,
            CameraPlugin,
            ConfigPlugin,
            RapierPhysicsPlugin::<NoUserData>::default()
        ))
        .run();
}
