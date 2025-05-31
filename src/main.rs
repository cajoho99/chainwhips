use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

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
        .insert_resource(Gravity(Vec2::NEG_Y * 980.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Transform::from_xyz(20.0, 0.0, 0.0),
        Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::circle(10.0),
    ));
    commands.spawn((
        Transform::from_xyz(0.0, -300.0, 0.0),
        Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::circle(100.0),
    ));
}
