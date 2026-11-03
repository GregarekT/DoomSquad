use std::{fs, io};

use minifb::{Key, Window};

const KEY_BINDINGS_FILE: &str = "key_bindings.txt";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[represent(usize)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    TurnLeft,
    TurnRight,
    AimUp,
    AimDown,
    Jump,
    RocketJump,
    Sprint,
    Crouch,
    Interact,
    ToggleAim,
    FireBullet,
    FireArrow,
    ToggleGodmode,
}

impl Action {
    pub const ALL: [self; 17] = [
        Self::MoveForward,
        Self::MoveBackward,
        Self::MoveLeft,
        Self::MoveRight,
        Self::TurnLeft,
        Self::TurnRight,
        Self::AimUp,
        Self::AimDown,
        Self::Jump,
        Self::RocketJump,
        Self::Sprint,
        Self::Crouch,
        Self::Interact,
        Self::ToggleAim,
        Self::FireBullet,
        Self::FireArrow,
        Self::ToggleGodmode,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::MoveForward => "Move Forward",
            Self::MoveBackward => "Move Backward",
            Self::MoveLeft => "Move Left",
            Self::MoveRight => "Move Right",
            Self::TurnLeft => "Turn Left",
            Self::TurnRight => "Turn Right",
            Self::AimUp => "Aim Up",
            Self::AimDown => "Aim Down",
            Self::Jump => "Jump",
            Self::RocketJump => "Rocket Jump",
            Self::Sprint => "Sprint",
            Self::Crouch => "Crouch",
            Self::Interact => "Interact",
            Self::ToggleAim => "Toggle Aim",
            Self::FireBullet => "Fire Bullet",
            Self::FireArrow => "Fire Arrow",
            Self::ToggleGodmode => "Toggle Godmode",
        }
    }
    fn config_name(self) -> &'static str {
        match self {
            Self::MoveForward => "move_forward",
            Self::MoveBackward => "move_backward",
            Self::MoveLeft => "move_left",
            Self::MoveRight => "move_right",
            Self::TurnLeft => "turn_left",
            Self::TurnRight => "turn_right",
            Self::AimUp => "aim_up",
            Self::AimDown => "aim_down",
            Self::Jump => "jump",
            Self::RocketJump => "rocket_jump",
            Self::Sprint => "sprint",
            Self::Crouch => "crouch",
            Self::Interact => "interact",
            Self::ToggleAim => "toggle_aim",
            Self::FireBullet => "fire_bullet",
            Self::FireArrow => "fire_arrow",
            Self::ToggleGodmode => "toggle_godmode",
        }
    }
    fn from_config_name(name:&str) -> Option<Self> {
        Self::ALL.iter().find(|&&action| action.config_name() == name)
    }
}

