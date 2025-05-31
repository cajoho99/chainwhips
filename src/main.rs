use std::time::Duration;

use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use delete_after::{DeleteAt, delete_at};
use input::controls;
use tilemap::helpers::tiled::TiledMap;

use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::*;

mod cursed_mouse_input;
mod delete_after;
mod input;
mod tilemap;

const GRAVITY: f32 = 980.0;
const CHAIN_LINK_COUNT: usize = 10;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ChainLink;

#[derive(Component, Clone, Copy)]
pub struct ChainBase {
    pos: f32,
}

impl ChainBase {
    pub fn moveLeft(&mut self) {
        self.pos = f32::max(-1.0, self.pos - 0.1);
    }
    pub fn moveRight(&mut self) {
        self.pos = f32::min(1.0, self.pos + 0.1);
    }
    pub fn getPos(self) -> Vec2 {
        let orig = Vec2::new(0.0, 35.0);
        let new = orig + Vec2::new(self.pos * 20.0, -15.0 * f32::abs(self.pos));
        new
    }
}

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
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
        ))
        .init_asset::<TiledMap>()
        .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
        .add_plugins(TilemapPlugin)
        .add_plugins(tilemap::helpers::tiled::TiledMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, tilemap::setup)
        .add_systems(
            FixedUpdate,
            (controls.in_set(TnuaUserControlsSystemSet), woosh_chain),
        )
        .add_systems(Update, camera_follow_player)
        .add_systems(Update, delete_at)
        .add_systems(Update, chainControll)
        .run();
}

fn chainControll(chainBases: Query<(&mut RevoluteJoint, &ChainBase)>) {
    for (mut joint, base) in chainBases {
        joint.local_anchor1 = base.getPos();
    }
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
            Transform::from_xyz(20.0, 0.1, 0.0),
            Sprite {
                image: asset_server.load("character.png"),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::capsule(32.0, 0.0),
            Friction::new(0.2),
            LockedAxes::ROTATION_LOCKED,
            Player,
            TnuaController::default(),
            // A sensor shape is not strictly necessary, but without it we'll get weird results.
            TnuaAvian2dSensorShape(Collider::rectangle(31.0, 31.0)),
        ))
        .id();

    spawn_chain(player, commands, asset_server);
}

fn spawn_chain(player: Entity, mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut chain_link = vec![
        commands
            .spawn((
                Transform::from_xyz(21.0, 500.1, 0.0).with_scale(Vec3::ONE * 0.1),
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
    ];
    for i in 1..CHAIN_LINK_COUNT {
        chain_link.push(
            commands
                .spawn((
                    ChainLink,
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

    commands.spawn((
        RevoluteJoint::new(player, chain_link[0])
            .with_local_anchor_1(Vec2::new(0.0, 50.0))
            .with_local_anchor_2(Vec2::new(0.0, 10.0)),
        ChainBase { pos: 0.0 },
    ));
}

fn camera_follow_player(
    players: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    _time: Res<Time>,
) {
    let mut min = Vec2::INFINITY;
    let mut max = Vec2::NEG_INFINITY;

    for player in players {
        min = min.min(player.translation.xy());
        max = max.max(player.translation.xy());
    }

    let center = (min + max) / 2.0;

    for mut camera in cameras {
        camera.translation.x = center.x;
        camera.translation.y = center.y;

        //const LERP_SPEED: f32 = 10.0;
        //camera.translation = camera
        //    .translation
        //    .lerp(player.translation, time.delta().as_secs_f32() * LERP_SPEED);
    }
}

fn woosh_chain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Single<&LinearVelocity, (With<Player>, Without<ChainLink>)>,
    chain_links: Query<(&LinearVelocity, &Transform), (With<ChainLink>, Without<Player>)>,
) {
    for (link_velocity, link_pos) in chain_links {
        let delta_v = link_velocity.0 - player.0;
        let velocity = delta_v.length();
        if velocity > 1000.0 {
            commands.spawn((
                Transform::from_translation(link_pos.translation),
                Sprite {
                    image: asset_server.load("ducky.png"),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                RigidBody::Dynamic,
                ExternalImpulse::new(delta_v / 2.0),
                Mass(1.0),
                DeleteAt::after(Duration::from_secs(5)),
            ));
        }
    }
}
