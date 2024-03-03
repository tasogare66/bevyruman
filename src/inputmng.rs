//use bevy::crate::bevy_input::button_input::ButtonInput;
use bevy::prelude::*;

#[derive(TypePath, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InputMngBtn {
    Up,
    Down,
    Left,
    Right,
    Shot,
    Dash,
    Melee,
}

pub fn startup_input_mng_system(mut commands: Commands) {
    commands.init_resource::<ButtonInput<InputMngBtn>>();
}

pub fn update_input_mng_system(
    kb: Res<ButtonInput<KeyCode>>,
    mb: Res<ButtonInput<MouseButton>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut input: ResMut<ButtonInput<InputMngBtn>>,
) {
    let mut is_left = kb.pressed(KeyCode::ArrowLeft) || kb.pressed(KeyCode::KeyA);
    let mut is_right = kb.pressed(KeyCode::ArrowRight) || kb.pressed(KeyCode::KeyD);
    let mut is_up = kb.pressed(KeyCode::ArrowUp) || kb.pressed(KeyCode::KeyW);
    let mut is_down = kb.pressed(KeyCode::ArrowDown) || kb.pressed(KeyCode::KeyS);
    let mut is_shot = kb.pressed(KeyCode::KeyZ) || mb.pressed(MouseButton::Left);
    let is_dash = kb.pressed(KeyCode::KeyX) || mb.pressed(MouseButton::Right);
    let is_melee = kb.pressed(KeyCode::KeyC);

    if let Some(gp) = gamepads.iter().next() {
        let threshold: f32 = 0.4;
        let left_stick_x = axes
            .get(GamepadAxis::new(gp, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x > threshold {
            is_right |= true;
        }
        if left_stick_x < -threshold {
            is_left |= true;
        }
        let left_stick_y = axes
            .get(GamepadAxis::new(gp, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y > threshold {
            is_up |= true;
        }
        if left_stick_y < -threshold {
            is_down |= true;
        }
        let right_stick_x = axes
            .get(GamepadAxis::new(gp, GamepadAxisType::RightStickX))
            .unwrap();
        let right_stick_y = axes
            .get(GamepadAxis::new(gp, GamepadAxisType::RightStickX))
            .unwrap();
        let dir: Vec2 = Vec2::new(right_stick_x, right_stick_y);
        if dir.length() > threshold {
            is_shot |= true;
        }
    }

    if is_left {
        input.press(InputMngBtn::Left);
    } else {
        input.release(InputMngBtn::Left);
    }
    if is_right {
        input.press(InputMngBtn::Right);
    } else {
        input.release(InputMngBtn::Right);
    }
    if is_up {
        input.press(InputMngBtn::Up);
    } else {
        input.release(InputMngBtn::Up);
    }
    if is_down {
        input.press(InputMngBtn::Down);
    } else {
        input.release(InputMngBtn::Down);
    }
    //shot
    if is_shot {
        input.press(InputMngBtn::Shot);
    } else {
        input.release(InputMngBtn::Shot);
    }
    //dash
    if is_dash {
        input.press(InputMngBtn::Dash);
    } else {
        input.release(InputMngBtn::Dash);
    }
    //melee
    if is_melee {
        input.press(InputMngBtn::Melee);
    } else {
        input.release(InputMngBtn::Melee);
    }
}
