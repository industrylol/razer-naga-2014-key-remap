use serde::{de::Error, Deserialize};
use std::ffi::c_int;
use std::fs;
use std::{collections::HashMap, ops::Deref};
use uinput::event::{
    keyboard::{Key, KeyPad},
    Release,
};
use uinput::event::{Code, Kind, Press};

#[derive(Copy, Clone)]
pub struct KeyMapper {
    pub(crate) keys: [Input; 12],
}
impl Default for KeyMapper {
    fn default() -> Self {
        Self {
            keys: [
                Key::_1.into(),
                Key::_2.into(),
                Key::_3.into(),
                Key::_4.into(),
                Key::_5.into(),
                Key::_6.into(),
                Key::_7.into(),
                Key::_8.into(),
                Key::_9.into(),
                Key::_0.into(),
                Key::Minus.into(),
                Key::Equal.into(),
            ],
        }
    }
}
impl KeyMapper {
    pub fn read_from_file(path: &str) -> Result<KeyMapper, String> {
        let contents = read_file_contents(path)?;
        let config: Config = toml::from_str(contents.as_str()).map_err(|e| format!("{}", e))?;

        let mut key_mapper = KeyMapper::default();

        for (from_key, to_key) in config.keys {
            let key_num = from_key
                .parse::<usize>()
                .map(|i| i - 1)
                .map_err(|e| format!("{}", e))?;

            if key_num >= 12 {
                return Err(format!("Invalid key number: {}", key_num));
            }

            key_mapper.keys[key_num] = to_key;
        }

        Ok(key_mapper)
    }
}

fn read_file_contents(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{}", e))
}

#[derive(Deserialize)]
struct Config {
    keys: HashMap<String, Input>,
}

#[derive(Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum Input {
    InputKey(InputKey),
    InputKeyPad(InputKeyPad),
}
impl Press for Input {}
impl Release for Input {}
impl Kind for Input {
    fn kind(&self) -> c_int {
        match self {
            Input::InputKey(k) => k.kind(),
            Input::InputKeyPad(kp) => kp.kind(),
        }
    }
}
impl Code for Input {
    fn code(&self) -> c_int {
        match self {
            Input::InputKey(k) => k.code(),
            Input::InputKeyPad(kp) => kp.code(),
        }
    }
}
impl From<Key> for Input {
    fn from(value: Key) -> Self {
        Self::InputKey(InputKey(value))
    }
}
impl From<KeyPad> for Input {
    fn from(value: KeyPad) -> Self {
        Self::InputKeyPad(InputKeyPad(value))
    }
}

#[derive(Copy, Clone)]
pub struct InputKey(Key);
impl Deref for InputKey {
    type Target = Key;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'de> Deserialize<'de> for InputKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;
        Ok(match variant.as_str() {
            "Reserved" => Self(Key::Reserved),
            "Esc" => Self(Key::Esc),
            "_1" => Self(Key::_1),
            "_2" => Self(Key::_2),
            "_3" => Self(Key::_3),
            "_4" => Self(Key::_4),
            "_5" => Self(Key::_5),
            "_6" => Self(Key::_6),
            "_7" => Self(Key::_7),
            "_8" => Self(Key::_8),
            "_9" => Self(Key::_9),
            "_0" => Self(Key::_0),
            "Minus" => Self(Key::Minus),
            "Equal" => Self(Key::Equal),
            "BackSpace" => Self(Key::BackSpace),
            "Tab" => Self(Key::Tab),
            "Q" => Self(Key::Q),
            "W" => Self(Key::W),
            "E" => Self(Key::E),
            "R" => Self(Key::R),
            "T" => Self(Key::T),
            "Y" => Self(Key::Y),
            "U" => Self(Key::U),
            "I" => Self(Key::I),
            "O" => Self(Key::O),
            "P" => Self(Key::P),
            "LeftBrace" => Self(Key::LeftBrace),
            "RightBrace" => Self(Key::RightBrace),
            "Enter" => Self(Key::Enter),
            "LeftControl" => Self(Key::LeftControl),
            "A" => Self(Key::A),
            "S" => Self(Key::S),
            "D" => Self(Key::D),
            "F" => Self(Key::F),
            "G" => Self(Key::G),
            "H" => Self(Key::H),
            "J" => Self(Key::J),
            "K" => Self(Key::K),
            "L" => Self(Key::L),
            "SemiColon" => Self(Key::SemiColon),
            "Apostrophe" => Self(Key::Apostrophe),
            "Grave" => Self(Key::Grave),
            "LeftShift" => Self(Key::LeftShift),
            "BackSlash" => Self(Key::BackSlash),
            "Z" => Self(Key::Z),
            "X" => Self(Key::X),
            "C" => Self(Key::C),
            "V" => Self(Key::V),
            "B" => Self(Key::B),
            "N" => Self(Key::N),
            "M" => Self(Key::M),
            "Comma" => Self(Key::Comma),
            "Dot" => Self(Key::Dot),
            "Slash" => Self(Key::Slash),
            "RightShift" => Self(Key::RightShift),
            "LeftAlt" => Self(Key::LeftAlt),
            "Space" => Self(Key::Space),
            "CapsLock" => Self(Key::CapsLock),
            "F1" => Self(Key::F1),
            "F2" => Self(Key::F2),
            "F3" => Self(Key::F3),
            "F4" => Self(Key::F4),
            "F5" => Self(Key::F5),
            "F6" => Self(Key::F6),
            "F7" => Self(Key::F7),
            "F8" => Self(Key::F8),
            "F9" => Self(Key::F9),
            "F10" => Self(Key::F10),
            "NumLock" => Self(Key::NumLock),
            "ScrollLock" => Self(Key::ScrollLock),
            "F11" => Self(Key::F11),
            "F12" => Self(Key::F12),
            "RightControl" => Self(Key::RightControl),
            "SysRq" => Self(Key::SysRq),
            "RightAlt" => Self(Key::RightAlt),
            "LineFeed" => Self(Key::LineFeed),
            "Home" => Self(Key::Home),
            "Up" => Self(Key::Up),
            "PageUp" => Self(Key::PageUp),
            "Left" => Self(Key::Left),
            "Right" => Self(Key::Right),
            "End" => Self(Key::End),
            "Down" => Self(Key::Down),
            "PageDown" => Self(Key::PageDown),
            "Insert" => Self(Key::Insert),
            "Delete" => Self(Key::Delete),
            "LeftMeta" => Self(Key::LeftMeta),
            "RightMeta" => Self(Key::RightMeta),
            "ScrollUp" => Self(Key::ScrollUp),
            "ScrollDown" => Self(Key::ScrollDown),
            "F13" => Self(Key::F13),
            "F14" => Self(Key::F14),
            "F15" => Self(Key::F15),
            "F16" => Self(Key::F16),
            "F17" => Self(Key::F17),
            "F18" => Self(Key::F18),
            "F19" => Self(Key::F19),
            "F20" => Self(Key::F20),
            "F21" => Self(Key::F21),
            "F22" => Self(Key::F22),
            "F23" => Self(Key::F23),
            "F24" => Self(Key::F24),
            invalid => {
                return Err(D::Error::custom(format!(
                    "Invalid keycode - {invalid}: cannot deserialize"
                )))
            }
        })
    }
}

#[derive(Copy, Clone)]
pub struct InputKeyPad(KeyPad);
impl Deref for InputKeyPad {
    type Target = KeyPad;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'de> Deserialize<'de> for InputKeyPad {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;
        Ok(match variant.as_str() {
            "KP::Asterisk" => Self(KeyPad::Asterisk),
            "KP::_7" => Self(KeyPad::_7),
            "KP::_8" => Self(KeyPad::_8),
            "KP::_9" => Self(KeyPad::_9),
            "KP::Minus" => Self(KeyPad::Minus),
            "KP::_4" => Self(KeyPad::_4),
            "KP::_5" => Self(KeyPad::_5),
            "KP::_6" => Self(KeyPad::_6),
            "KP::Plus" => Self(KeyPad::Plus),
            "KP::_1" => Self(KeyPad::_1),
            "KP::_2" => Self(KeyPad::_2),
            "KP::_3" => Self(KeyPad::_3),
            "KP::_0" => Self(KeyPad::_0),
            "KP::Dot" => Self(KeyPad::Dot),
            "KP::AltComma" => Self(KeyPad::AltComma),
            "KP::Enter" => Self(KeyPad::Enter),
            "KP::Slash" => Self(KeyPad::Slash),
            "KP::Equal" => Self(KeyPad::Equal),
            "KP::PlusMinus" => Self(KeyPad::PlusMinus),
            "KP::Comma" => Self(KeyPad::Comma),
            "KP::LeftParen" => Self(KeyPad::LeftParen),
            "KP::RightParen" => Self(KeyPad::RightParen),
            invalid => {
                return Err(D::Error::custom(format!(
                    "Invalid keycode - {invalid}: cannot deserialize"
                )))
            }
        })
    }
}
