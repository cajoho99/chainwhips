use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use input::{gamepad_system, keyboard_and_mouse_system};
use tilemap::helpers::tiled::TiledMap;

mod cursed_mouse_input;
mod input;
mod tilemap;

const GRAVITY: f32 = 980.0;
const CHAIN_LINK_COUNT: usize = 15;

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugins::default(),
            //PhysicsDebugPlugin::default(),
        ))
        .init_asset::<TiledMap>()
        .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
        .add_plugins(TilemapPlugin)
        .add_plugins(tilemap::helpers::tiled::TiledMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, tilemap::setup)
        .add_systems(Update, (gamepad_system, keyboard_and_mouse_system))
        .add_systems(Update, camera_follow_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let player_scale = 0.1;

    // DUCK
    commands.spawn((
        Transform::from_xyz(20.0, 0.0, 0.0).with_scale(Vec3::ONE * player_scale),
        Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        },
        ExternalImpulse::ZERO,
        RigidBody::Dynamic,
        Collider::circle(10.0 / player_scale),
        Mass(1.0), // TODO
    ));

    // Music
    commands.spawn((
        AudioPlayer::new(asset_server.load("ost.ogg")),
        PlaybackSettings::LOOP,
    ));

    // Player
    let player = commands
        .spawn((
            Transform::from_xyz(20.0, 0.1, 0.0).with_scale(Vec3::ONE * 0.1),
            Sprite {
                image: asset_server.load("character.png"),
                custom_size: Some(Vec2::new(1000.0, 1000.0)),
                ..Default::default()
            },
            ExternalImpulse::ZERO,
            RigidBody::Dynamic,
            Collider::rectangle(550.0, 550.0),
            LockedAxes::ROTATION_LOCKED,
            Mass(1.0),
            Player,
        ))
        .id();

    spawn_chain(player, commands, asset_server);
}

fn spawn_chain(player: Entity, mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut chain_link = Vec::new();
    for i in 0..CHAIN_LINK_COUNT {
        chain_link.push(
            commands
                .spawn((
                    Transform::from_xyz(21.0, i as f32 + 500.1, 0.0).with_scale(Vec3::ONE * 0.1),
                    RigidBody::Dynamic,
                    ExternalImpulse::ZERO,
                    Collider::capsule(75.0, 80.0),
                    Sprite {
                        image: asset_server.load("chain.png"),
                        custom_size: Some(Vec2::new(100.0, 200.0)),
                        ..Default::default()
                    },
                    Mass(0.001),
                ))
                .id(),
        );
    }

    for i in 0..(CHAIN_LINK_COUNT - 1) {
        let c1 = chain_link[i];
        let c2 = chain_link[i + 1];
        commands.spawn(
            RevoluteJoint::new(c1, c2)
                .with_local_anchor_1(Vec2::new(0.0, -10.0))
                .with_local_anchor_2(Vec2::new(0.0, 10.0)),
        );
    }

    commands.spawn(
        RevoluteJoint::new(player, chain_link[0])
            .with_local_anchor_1(Vec2::new(0.0, 50.0))
            .with_local_anchor_2(Vec2::new(0.0, 10.0)),
    );
}

fn camera_follow_player(
    players: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    _time: Res<Time>,
) {
    if let Some(player) = players.into_iter().next() {
        for mut camera in cameras {
            camera.translation = player.translation;

            //const LERP_SPEED: f32 = 10.0;
            //camera.translation = camera
            //    .translation
            //    .lerp(player.translation, time.delta().as_secs_f32() * LERP_SPEED);
        }
    }
}
