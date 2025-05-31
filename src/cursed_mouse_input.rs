use avian2d::prelude::{ExternalImpulse, RigidBody, RigidBodyQuery};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, sprite::Sprite};

pub fn player_movement(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,

    players: Query<&mut ExternalImpulse, With<Sprite>>,
) {
    let factor = 2.0;
    for (mut impulse) in players {
        let direction = accumulated_mouse_motion.delta * Vec2::new(1.0, -1.0);
        impulse.apply_impulse(direction * factor);
    }
}
