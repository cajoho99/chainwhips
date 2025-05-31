use bevy::prelude::*;
use bevy_tnua::{
    builtins::TnuaBuiltinDash,
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
};

use crate::ChainBase;

pub fn controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut TnuaController>,
    mut gamepads: Query<&Gamepad>,
    mut base: Query<&mut ChainBase>,
) {
    let Ok(mut controller) = player.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    if let Ok(gamepad) = gamepads.single_mut() {
        if gamepad.pressed(GamepadButton::DPadLeft) {
            direction -= Vec3::X;
        }
        if gamepad.pressed(GamepadButton::DPadRight) {
            direction += Vec3::X;
        }
        if gamepad.pressed(GamepadButton::DPadUp) {
            jump(&mut controller);
        }
        if gamepad.pressed(GamepadButton::DPadDown) {
            slam(&mut controller);
        }
        if gamepad.right_stick().x > 0.8 {
            base.iter_mut().for_each(|mut base| base.moveRight());
        }
        if gamepad.right_stick().x < -0.8 {
            base.iter_mut().for_each(|mut base| base.moveLeft());
        }
    } else {
        if keyboard.pressed(KeyCode::KeyA) {
            direction -= Vec3::X;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }
        if keyboard.pressed(KeyCode::Space) {
            jump(&mut controller);
        }
        if keyboard.pressed(KeyCode::KeyS) {
            slam(&mut controller);
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            base.iter_mut().for_each(|mut base| base.moveRight());
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            base.iter_mut().for_each(|mut base| base.moveLeft());
        }
    }

    walk(controller, direction);
}

fn walk(mut controller: Mut<'_, TnuaController>, direction: Vec3) {
    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 4000.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 17.0,
        acceleration: 600.0,
        air_acceleration: 600.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });
}

fn jump(controller: &mut Mut<'_, TnuaController>) {
    controller.action(TnuaBuiltinJump {
        // The height is the only mandatory field of the jump button.
        height: 32.0 * 10.0,
        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
        ..Default::default()
    });
}

fn slam(controller: &mut Mut<'_, TnuaController>) {
    controller.action(TnuaBuiltinDash {
        displacement: -Vec3::Y * 1000.0,
        desired_forward: None,
        allow_in_air: true,
        speed: 1000000.00,
        brake_to_speed: 0.0, // TODO,
        acceleration: 10000.0,
        brake_acceleration: 100.0,
        input_buffer_time: 0.0,
    });
}
