use bevy::prelude::*;

use crate::config::Config;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Resource, Debug)]
pub struct MainPlayer(pub u32);

#[derive(Component, Debug)]
pub struct Player(pub u32);

#[derive(Bundle)]
struct PlayerBundle {
    speed: Speed,
    pbr: PbrBundle,
    player: Player,
}

#[derive(Component)]
struct Speed(f32);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_id = 1;
    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    let player = PlayerBundle {
        speed: Speed(2.5),
        pbr,
        player: Player(player_id),
    };

    commands.insert_resource(MainPlayer(player_id));
    commands.spawn(player);
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut players: Query<(&mut Transform, &Speed, &Player), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    main_player: Option<Res<MainPlayer>>,
) {
    let cam_r = cam_q.get_single();
    let mp_o = main_player.and_then(|mp| {
        players
            .iter_mut()
            .find(|(_, _, p)| p.0 == mp.0)
    });

    if let (Ok(cam), Some(mp)) = (cam_r, mp_o) {
        let (mut player_transform, player_speed, _) = mp;

        let mut direction = keys.get_pressed().fold(Vec3::ZERO, |_, k| match k {
            KeyCode::W => cam.forward(),
            KeyCode::A => cam.left(),
            KeyCode::S => cam.back(),
            KeyCode::D => cam.right(),
            _ => Vec3::ZERO,
        });

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();

        player_transform.translation += movement;
    }

}
