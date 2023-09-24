use bevy::{input::mouse::MouseMotion, prelude::*};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

use crate::player::{MainPlayer, Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookTransformPlugin)
            .add_systems(Update, camera_follow)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let eye = Vec3::new(-2.0, 2.5, 5.0);
    let target = Vec3::ZERO;

    commands
        .spawn(LookTransformBundle {
            transform: LookTransform::new(eye, target, Vec3::Y),
            smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
        })
        .insert(Camera3dBundle::default());
}

fn camera_follow(
    main_player: Option<Res<MainPlayer>>,
    players: Query<(&Player, &Transform), Without<Camera>>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    info!("main player: {:?}", main_player);
    let cam_r = cameras.get_single_mut();
    let mp_o = main_player.and_then(|mp| players.iter().find(|(p, _)| p.0 == mp.0));
    info!("main player option {:?}", mp_o);

    if let (Ok(mut cam), Some(mp)) = (cam_r, mp_o) {
        let (_, pt) = mp;
        let pos = pt.translation;

        info!("position {:?}", pos);
        info!("cam pos before {:?}", cam.translation);
        cam.translation.x = pos.x;
        cam.translation.y = pos.y;
        cam.translation.z = pos.z;
        info!("cam pos after {:?}", cam.translation);
    }
}

fn camera_control(
    mut cams: Query<&mut Transform, With<Camera>>,
    mtn_evr: EventReader<MouseMotion>,
) {
}
