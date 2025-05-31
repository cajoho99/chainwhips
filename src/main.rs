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
        .add_systems(
            Update,
            (
                camera_follow_player,
                gamepad_system,
                keyboard_and_mouse_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let player_scale = 0.1;
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

    commands.spawn((
        Transform::from_xyz(20.0, 0.1, 0.0).with_scale(Vec3::ONE * 0.1),
        Sprite {
            image: asset_server.load("character.png"),
            custom_size: Some(Vec2::new(1000.0, 1000.0)),
            ..Default::default()
        },
        ExternalImpulse::ZERO,
        RigidBody::Dynamic,
        Collider::rectangle(100.0, 100.0),
        ColliderDensity(0.01),
        LockedAxes::ROTATION_LOCKED,
        Player,
    ));
}

fn camera_follow_player(
    players: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Some(player) = players.into_iter().next() {
        for mut camera in cameras {
            camera.translation = player.translation;
        }
    }
}
