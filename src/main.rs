use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use input::{gamepad_system, keyboard_and_mouse_system};
use tilemap::helpers::tiled::TiledMap;
use tilemap::swap_texture_or_hide;

mod cursed_mouse_input;
mod input;
mod tilemap;

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
        ))
        .init_asset::<TiledMap>()
        .insert_resource(Gravity(Vec2::NEG_Y * 980.0))
        .add_plugins(TilemapPlugin)
        .add_plugins(tilemap::helpers::tiled::TiledMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, tilemap::setup)
        //.add_systems(Update, helpers::camera::movement)
        .add_systems(Update, cursed_mouse_input::player_movement)
        .add_systems(
            Update,
            (
                swap_texture_or_hide,
                gamepad_system,
                keyboard_and_mouse_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Transform::from_xyz(20.0, 0.0, 0.0).with_scale(Vec3::ONE * 0.1),
        Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        },
        ExternalImpulse::ZERO,
        RigidBody::Dynamic,
        Collider::circle(10.0),
    ));
}
