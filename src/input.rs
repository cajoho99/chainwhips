use avian2d::prelude::ExternalImpulse;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};

use crate::Player;

pub fn gamepad_system(gamepads: Query<(Entity, &Gamepad)>) {
    for (entity, gamepad) in &gamepads {
        if gamepad.just_pressed(GamepadButton::South) {
            info!("{} just pressed South", entity);
        } else if gamepad.just_released(GamepadButton::South) {
            info!("{} just released South", entity);
        }

        let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{} RightTrigger2 value is {}", entity, right_trigger);
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{} LeftStickX value is {}", entity, left_stick_x);
        }
    }
}

pub fn keyboard_and_mouse_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<AccumulatedMouseMotion>,
    players: Query<&mut ExternalImpulse, With<Player>>,
) {
    // if keyboard_input.just_pressed(KeyCode::KeyA) {
    //     info!("'A' just pressed");
    // }
    // if keyboard_input.just_released(KeyCode::KeyA) {
    //     info!("'A' just released");
    // }

    if mouse_input.delta != Vec2::ZERO {
        let delta = mouse_input.delta;
        info!("mouse moved ({}, {})", delta.x, delta.y);
    }

    let move_factor = 50.0;
    let jump_factor = 400.0;

    for mut impulse in players {
        if keyboard_input.pressed(KeyCode::KeyA) {
            impulse.apply_impulse(Vec2::new(-1.0, 0.0) * move_factor);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            impulse.apply_impulse(Vec2::new(1.0, 0.0) * move_factor);
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            impulse.apply_impulse(Vec2::new(0.0, 1.0) * jump_factor);
        }
    }
}

pub fn key_controls(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut TnuaController>) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }

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

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 32.0 * 10.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}
