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
    LockEnemies,
    ExitGame_or_Menu,
}

impl Action {
    pub const ALL: [self; 19]= [
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
        Self::LockEnemies,
        Self::ExitGame_or_Menu,
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
            Self::LockEnemies => "Lock Enemies",
            Self::ExitGame_or_Menu => "Exit Game or Menu",
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
            Self::LockEnemies => "lock_enemies",
            Self::ExitGame_or_Menu => "exit_game_or_menu",
        }
    }
    fn from_config_name(name:&str) -> Option<Self> {
        Self::ALL.iter().find(|&&action| action.config_name() == name)
    }
}


#[derive(Clone)]
pub struct KeyBindings {
    keys: [Key; Action::ALL.len()],
}

impl KeyBindings {
    pub fn load() -> Self {
        let mut bindings = Self::default();
        match fs::read_to_string(KEY_BINDINGS_FILE) {
            Ok (contents) => {
                for (line_number, line) in contents.lines().enumerate() {
                    let Some((action_name, key_name)) = line.split_once('=') else {
                        eprintln!(
                            "Invalid line format in key bindings file at line {KEY_BINDINGS_PATH}: {}",
                            line_number + 1
                        );
                        continue;
                    };
                    let action = Action::from_config_name(action_name);
                    let key = ALL_KEYS.iter().copied.find(|key| format!("{key:?}") == key_name);
                    match (action, key) {
                        (Some(action), Some(key)) => bindings.set (action, key), _ => eprintln!(
                            "Invalid action or key in key bindings file at line {KEY_BINDINGS_PATH}: {}",
                            line_number + 1
                        ),
                        
                    }
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => eprintln!("Failed to read {KEY_BINDINGS_PATH}: {error}"),
        }
        bindings
    }
    pub fn get (&self, action: Action) -> Key {
        self.keys[action as usize]
    }
    pub fn set (&mut self, action: Action, key: Key) {
        self.keys[action as usize] = key;
    }
    pub fn is_down(&self, action: Action, window: &Window) -> bool {
        window.is_key_down(self.get(action))
    }
    pub fn is_pressed(&self, action: Action, window: &Window) -> bool {
        window.is_key_pressed(self.get(action), minifb::KeyRepeat::No)
    }
    pub fn save(&self) -> io::Result <()> {
        let contents = Action::ALL.iter().map(|action| format!("{}={:?}", action.config_name(), self.get(action))).collect::<Vec<_>>().join("\n");
        fs::write(KEY_BINDINGS_FILE, contents)
    }
}


// default key bindings for each action
impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            keys: [
                Key::W,
                Key::S,
                Key::A,
                Key::D,
                Key::Left,
                Key::Right,
                Key::Up,
                Key::Down,
                Key::Space,
                Key::R,
                Key::LeftShift,
                Key::C,
                Key::F,
                Key::E,
                Key::RightCtrl,
                Key::RightShift,
                Key::G,
                Key::L,
                Key::Escape,
            ],
        }
    }
}


const ALL_KEYS: &[Keys] = &[
    Key::Key0,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
    Key::Key9,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Up,
    Key::Apostrophe,
    Key::Backquote,
    Key::Backslash,
    Key::Comma,
    Key::Equal,
    Key::LeftBracket,
    Key::Minus,
    Key::Period,
    Key::RightBracket,
    Key::Semicolon,
    Key::Slash,
    Key::Backspace,
    Key::Delete,
    Key::End,
    Key::Enter,
    Key::Escape,
    Key::Home,
    Key::Insert,
    Key::Menu,
    Key::PageDown,
    Key::PageUp,
    Key::Pause,
    Key::Space,
    Key::Tab,
    Key::NumLock,
    Key::CapsLock,
    Key::ScrollLock,
    Key::LeftShift,
    Key::RightShift,
    Key::LeftCtrl,
    Key::RightCtrl,
    Key::NumPad0,
    Key::NumPad1,
    Key::NumPad2,
    Key::NumPad3,
    Key::NumPad4,
    Key::NumPad5,
    Key::NumPad6,
    Key::NumPad7,
    Key::NumPad8,
    Key::NumPad9,
    Key::NumPadDot,
    Key::NumPadSlash,
    Key::NumPadAsterisk,
    Key::NumPadMinus,
    Key::NumPadPlus,
    Key::NumPadEnter,
    Key::LeftAlt,
    Key::RightAlt,
    Key::LeftSuper,
    Key::RightSuper,
    Key::Unknown,
];
