use bevy_input::{keyboard::{KeyCode, Key, NativeKeyCode, NativeKey}, mouse::MouseButton};
use miniquad as mq;

pub fn convert_mouse_button(mouse_button: mq::MouseButton) -> MouseButton {
    match mouse_button {
        mq::MouseButton::Left => MouseButton::Left,
        mq::MouseButton::Right => MouseButton::Right,
        mq::MouseButton::Middle => MouseButton::Middle,
        mq::MouseButton::Unknown => MouseButton::Other(0),
    }
}

pub fn key_code_to_unprintable_logical_key(key_code: KeyCode) -> Option<Key> {
    return Some(match key_code {
        KeyCode::Unidentified(native) => Key::Unidentified(match native {
            NativeKeyCode::Unidentified => NativeKey::Unidentified,
            NativeKeyCode::Android(k) => NativeKey::Android(k),
            NativeKeyCode::MacOS(k) => NativeKey::MacOS(k),
            NativeKeyCode::Windows(k) => NativeKey::Windows(k),
            NativeKeyCode::Xkb(k) => NativeKey::Xkb(k),
        }),
        KeyCode::AltLeft => Key::Alt,
        KeyCode::AltRight => Key::Alt,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::CapsLock => Key::CapsLock,
        KeyCode::ContextMenu => Key::ContextMenu,
        KeyCode::ControlLeft => Key::Control,
        KeyCode::ControlRight => Key::Control,
        KeyCode::Enter => Key::Enter,
        KeyCode::SuperLeft => Key::Super,
        KeyCode::SuperRight => Key::Super,
        KeyCode::ShiftLeft => Key::Shift,
        KeyCode::ShiftRight => Key::Shift,
        KeyCode::Convert => Key::Convert,
        KeyCode::KanaMode => Key::KanaMode,
        KeyCode::Lang1 => return None,
        KeyCode::Lang2 => return None,
        KeyCode::Lang3 => return None,
        KeyCode::Lang4 => return None,
        KeyCode::Lang5 => return None,
        KeyCode::NonConvert => Key::NonConvert,
        KeyCode::Delete => Key::Delete, 
        KeyCode::End => Key::End,
        KeyCode::Help => Key::Help,
        KeyCode::Home => Key::Home,
        KeyCode::Insert => Key::Insert,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::ArrowDown => Key::ArrowDown,
        KeyCode::ArrowLeft => Key::ArrowLeft,
        KeyCode::ArrowRight => Key::ArrowRight,
        KeyCode::ArrowUp => Key::ArrowUp,
        KeyCode::NumLock => Key::NumLock,
        KeyCode::NumpadBackspace => Key::Backspace,
        KeyCode::NumpadClear => Key::Clear,
        KeyCode::NumpadClearEntry => return None,
        KeyCode::NumpadEnter => Key::Enter,
        KeyCode::NumpadMemoryAdd => return None,
        KeyCode::NumpadMemoryClear => return None,
        KeyCode::NumpadMemoryRecall => return None,
        KeyCode::NumpadMemoryStore => return None,
        KeyCode::Escape => Key::Escape,
        KeyCode::Fn => Key::Fn,
        KeyCode::FnLock => Key::FnLock,
        KeyCode::PrintScreen => Key::PrintScreen,
        KeyCode::ScrollLock => Key::ScrollLock,
        KeyCode::Pause => Key::Pause,
        KeyCode::BrowserBack => Key::BrowserBack,
        KeyCode::BrowserFavorites => Key::BrowserFavorites,
        KeyCode::BrowserForward => Key::BrowserForward,
        KeyCode::BrowserHome => Key::BrowserHome,
        KeyCode::BrowserRefresh => Key::BrowserRefresh,
        KeyCode::BrowserSearch => Key::BrowserSearch,
        KeyCode::BrowserStop => Key::BrowserStop,
        KeyCode::Eject => Key::Eject,
        KeyCode::LaunchApp1 => Key::LaunchApplication1,
        KeyCode::LaunchApp2 => Key::LaunchApplication2,
        KeyCode::LaunchMail => Key::LaunchMail,
        KeyCode::MediaPlayPause => Key::MediaPlayPause,
        KeyCode::MediaSelect => return None,
        KeyCode::MediaStop => Key::MediaStop,
        KeyCode::MediaTrackNext => Key::MediaTrackNext,
        KeyCode::MediaTrackPrevious => Key::MediaTrackPrevious,
        KeyCode::Power => Key::Power,
        KeyCode::Sleep => Key::Standby,
        KeyCode::AudioVolumeDown => Key::AudioVolumeDown,
        KeyCode::AudioVolumeMute => Key::AudioVolumeMute,
        KeyCode::AudioVolumeUp => Key::AudioVolumeUp,
        KeyCode::WakeUp => Key::WakeUp,
        KeyCode::Meta => Key::Meta,
        KeyCode::Hyper => Key::Hyper,
        KeyCode::Turbo => return None,
        KeyCode::Abort => return None,
        KeyCode::Resume => return None,
        KeyCode::Suspend => return None,
        KeyCode::Again => Key::Again,
        KeyCode::Copy => Key::Copy,
        KeyCode::Cut => Key::Cut,
        KeyCode::Find => Key::Find,
        KeyCode::Open => Key::Open,
        KeyCode::Paste => Key::Paste,
        KeyCode::Props => Key::Props,
        KeyCode::Select => Key::Select,
        KeyCode::Undo => Key::Undo,
        KeyCode::Hiragana => Key::Hiragana,
        KeyCode::Katakana => Key::Katakana,
        KeyCode::F1 => Key::F1,
        KeyCode::F2 => Key::F2,
        KeyCode::F3 => Key::F3,
        KeyCode::F4 => Key::F4,
        KeyCode::F5 => Key::F5,
        KeyCode::F6 => Key::F6,
        KeyCode::F7 => Key::F7,
        KeyCode::F8 => Key::F8,
        KeyCode::F9 => Key::F9,
        KeyCode::F10 => Key::F10,
        KeyCode::F11 => Key::F11,
        KeyCode::F12 => Key::F12,
        KeyCode::F13 => Key::F13,
        KeyCode::F14 => Key::F14,
        KeyCode::F15 => Key::F15,
        KeyCode::F16 => Key::F16,
        KeyCode::F17 => Key::F17,
        KeyCode::F18 => Key::F18,
        KeyCode::F19 => Key::F19,
        KeyCode::F20 => Key::F20,
        KeyCode::F21 => Key::F21,
        KeyCode::F22 => Key::F22,
        KeyCode::F23 => Key::F23,
        KeyCode::F24 => Key::F24,
        KeyCode::F25 => Key::F25,
        KeyCode::F26 => Key::F26,
        KeyCode::F27 => Key::F27,
        KeyCode::F28 => Key::F28,
        KeyCode::F29 => Key::F29,
        KeyCode::F30 => Key::F30,
        KeyCode::F31 => Key::F31,
        KeyCode::F32 => Key::F32,
        KeyCode::F33 => Key::F33,
        KeyCode::F34 => Key::F34,
        KeyCode::F35 => Key::F35,
        _ => return None,
    })
}

pub fn key_code_is_printable(key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Unidentified(..) |
        KeyCode::AltLeft |
        KeyCode::AltRight |
        KeyCode::Backspace |
        KeyCode::CapsLock |
        KeyCode::ContextMenu |
        KeyCode::ControlLeft |
        KeyCode::ControlRight |
        KeyCode::Enter |
        KeyCode::SuperLeft |
        KeyCode::SuperRight |
        KeyCode::ShiftLeft |
        KeyCode::ShiftRight |
        KeyCode::Convert |
        KeyCode::KanaMode |
        KeyCode::Lang1 |
        KeyCode::Lang2 |
        KeyCode::Lang3 |
        KeyCode::Lang4 |
        KeyCode::Lang5 |
        KeyCode::NonConvert |
        KeyCode::Delete |
        KeyCode::End |
        KeyCode::Help |
        KeyCode::Home |
        KeyCode::Insert |
        KeyCode::PageDown |
        KeyCode::PageUp |
        KeyCode::ArrowDown |
        KeyCode::ArrowLeft |
        KeyCode::ArrowRight |
        KeyCode::ArrowUp |
        KeyCode::NumLock |
        KeyCode::NumpadBackspace |
        KeyCode::NumpadClear |
        KeyCode::NumpadClearEntry |
        KeyCode::NumpadEnter |
        KeyCode::NumpadMemoryAdd |
        KeyCode::NumpadMemoryClear |
        KeyCode::NumpadMemoryRecall |
        KeyCode::NumpadMemoryStore |
        KeyCode::Escape |
        KeyCode::Fn |
        KeyCode::FnLock |
        KeyCode::PrintScreen |
        KeyCode::ScrollLock |
        KeyCode::Pause |
        KeyCode::BrowserBack |
        KeyCode::BrowserFavorites |
        KeyCode::BrowserForward |
        KeyCode::BrowserHome |
        KeyCode::BrowserRefresh |
        KeyCode::BrowserSearch |
        KeyCode::BrowserStop |
        KeyCode::Eject |
        KeyCode::LaunchApp1 |
        KeyCode::LaunchApp2 |
        KeyCode::LaunchMail |
        KeyCode::MediaPlayPause |
        KeyCode::MediaSelect |
        KeyCode::MediaStop |
        KeyCode::MediaTrackNext |
        KeyCode::MediaTrackPrevious |
        KeyCode::Power |
        KeyCode::Sleep |
        KeyCode::AudioVolumeDown |
        KeyCode::AudioVolumeMute |
        KeyCode::AudioVolumeUp |
        KeyCode::WakeUp |
        KeyCode::Meta |
        KeyCode::Hyper |
        KeyCode::Turbo |
        KeyCode::Abort |
        KeyCode::Resume |
        KeyCode::Suspend |
        KeyCode::Again |
        KeyCode::Copy |
        KeyCode::Cut |
        KeyCode::Find |
        KeyCode::Open |
        KeyCode::Paste |
        KeyCode::Props |
        KeyCode::Select |
        KeyCode::Undo |
        KeyCode::Hiragana |
        KeyCode::Katakana |
        KeyCode::F1 |
        KeyCode::F2 |
        KeyCode::F3 |
        KeyCode::F4 |
        KeyCode::F5 |
        KeyCode::F6 |
        KeyCode::F7 |
        KeyCode::F8 |
        KeyCode::F9 |
        KeyCode::F10 |
        KeyCode::F11 |
        KeyCode::F12 |
        KeyCode::F13 |
        KeyCode::F14 |
        KeyCode::F15 |
        KeyCode::F16 |
        KeyCode::F17 |
        KeyCode::F18 |
        KeyCode::F19 |
        KeyCode::F20 |
        KeyCode::F21 |
        KeyCode::F22 |
        KeyCode::F23 |
        KeyCode::F24 |
        KeyCode::F25 |
        KeyCode::F26 |
        KeyCode::F27 |
        KeyCode::F28 |
        KeyCode::F29 |
        KeyCode::F30 |
        KeyCode::F31 |
        KeyCode::F32 |
        KeyCode::F33 |
        KeyCode::F34 |
        KeyCode::F35 => false,
        _ => true,
    }
}

pub fn convert_virtual_key_code(key_code: mq::KeyCode) -> Option<KeyCode> {
    match key_code {
        mq::KeyCode::Key1 => Some(KeyCode::Digit1),
        mq::KeyCode::Key2 => Some(KeyCode::Digit2),
        mq::KeyCode::Key3 => Some(KeyCode::Digit3),
        mq::KeyCode::Key4 => Some(KeyCode::Digit4),
        mq::KeyCode::Key5 => Some(KeyCode::Digit5),
        mq::KeyCode::Key6 => Some(KeyCode::Digit6),
        mq::KeyCode::Key7 => Some(KeyCode::Digit7),
        mq::KeyCode::Key8 => Some(KeyCode::Digit8),
        mq::KeyCode::Key9 => Some(KeyCode::Digit9),
        mq::KeyCode::Key0 => Some(KeyCode::Digit0),
        mq::KeyCode::A => Some(KeyCode::KeyA),
        mq::KeyCode::B => Some(KeyCode::KeyB),
        mq::KeyCode::C => Some(KeyCode::KeyC),
        mq::KeyCode::D => Some(KeyCode::KeyD),
        mq::KeyCode::E => Some(KeyCode::KeyE),
        mq::KeyCode::F => Some(KeyCode::KeyF),
        mq::KeyCode::G => Some(KeyCode::KeyG),
        mq::KeyCode::H => Some(KeyCode::KeyH),
        mq::KeyCode::I => Some(KeyCode::KeyI),
        mq::KeyCode::J => Some(KeyCode::KeyJ),
        mq::KeyCode::K => Some(KeyCode::KeyK),
        mq::KeyCode::L => Some(KeyCode::KeyL),
        mq::KeyCode::M => Some(KeyCode::KeyM),
        mq::KeyCode::N => Some(KeyCode::KeyN),
        mq::KeyCode::O => Some(KeyCode::KeyO),
        mq::KeyCode::P => Some(KeyCode::KeyP),
        mq::KeyCode::Q => Some(KeyCode::KeyQ),
        mq::KeyCode::R => Some(KeyCode::KeyR),
        mq::KeyCode::S => Some(KeyCode::KeyS),
        mq::KeyCode::T => Some(KeyCode::KeyT),
        mq::KeyCode::U => Some(KeyCode::KeyU),
        mq::KeyCode::V => Some(KeyCode::KeyV),
        mq::KeyCode::W => Some(KeyCode::KeyW),
        mq::KeyCode::X => Some(KeyCode::KeyX),
        mq::KeyCode::Y => Some(KeyCode::KeyY),
        mq::KeyCode::Z => Some(KeyCode::KeyZ),
        mq::KeyCode::Escape => Some(KeyCode::Escape),
        mq::KeyCode::F1 => Some(KeyCode::F1),
        mq::KeyCode::F2 => Some(KeyCode::F2),
        mq::KeyCode::F3 => Some(KeyCode::F3),
        mq::KeyCode::F4 => Some(KeyCode::F4),
        mq::KeyCode::F5 => Some(KeyCode::F5),
        mq::KeyCode::F6 => Some(KeyCode::F6),
        mq::KeyCode::F7 => Some(KeyCode::F7),
        mq::KeyCode::F8 => Some(KeyCode::F8),
        mq::KeyCode::F9 => Some(KeyCode::F9),
        mq::KeyCode::F10 => Some(KeyCode::F10),
        mq::KeyCode::F11 => Some(KeyCode::F11),
        mq::KeyCode::F12 => Some(KeyCode::F12),
        mq::KeyCode::F13 => Some(KeyCode::F13),
        mq::KeyCode::F14 => Some(KeyCode::F14),
        mq::KeyCode::F15 => Some(KeyCode::F15),
        mq::KeyCode::F16 => Some(KeyCode::F16),
        mq::KeyCode::F17 => Some(KeyCode::F17),
        mq::KeyCode::F18 => Some(KeyCode::F18),
        mq::KeyCode::F19 => Some(KeyCode::F19),
        mq::KeyCode::F20 => Some(KeyCode::F20),
        mq::KeyCode::F21 => Some(KeyCode::F21),
        mq::KeyCode::F22 => Some(KeyCode::F22),
        mq::KeyCode::F23 => Some(KeyCode::F23),
        mq::KeyCode::F24 => Some(KeyCode::F24),
        mq::KeyCode::F25 => None,
        mq::KeyCode::PrintScreen => Some(KeyCode::PrintScreen),
        mq::KeyCode::ScrollLock => Some(KeyCode::ScrollLock),
        mq::KeyCode::Pause => Some(KeyCode::Pause),
        mq::KeyCode::Insert => Some(KeyCode::Insert),
        mq::KeyCode::Home => Some(KeyCode::Home),
        mq::KeyCode::Delete => Some(KeyCode::Delete),
        mq::KeyCode::End => Some(KeyCode::End),
        mq::KeyCode::PageDown => Some(KeyCode::PageDown),
        mq::KeyCode::PageUp => Some(KeyCode::PageUp),
        mq::KeyCode::Left => Some(KeyCode::ArrowLeft),
        mq::KeyCode::Up => Some(KeyCode::ArrowUp),
        mq::KeyCode::Right => Some(KeyCode::ArrowRight),
        mq::KeyCode::Down => Some(KeyCode::ArrowDown),
        mq::KeyCode::Backspace => Some(KeyCode::Backspace),
        mq::KeyCode::Enter => Some(KeyCode::Enter),
        mq::KeyCode::Space => Some(KeyCode::Space),
        mq::KeyCode::Menu => Some(KeyCode::ContextMenu),
        mq::KeyCode::NumLock => Some(KeyCode::NumLock),
        mq::KeyCode::Kp0 => Some(KeyCode::Numpad0),
        mq::KeyCode::Kp1 => Some(KeyCode::Numpad1),
        mq::KeyCode::Kp2 => Some(KeyCode::Numpad2),
        mq::KeyCode::Kp3 => Some(KeyCode::Numpad3),
        mq::KeyCode::Kp4 => Some(KeyCode::Numpad4),
        mq::KeyCode::Kp5 => Some(KeyCode::Numpad5),
        mq::KeyCode::Kp6 => Some(KeyCode::Numpad6),
        mq::KeyCode::Kp7 => Some(KeyCode::Numpad7),
        mq::KeyCode::Kp8 => Some(KeyCode::Numpad8),
        mq::KeyCode::Kp9 => Some(KeyCode::Numpad9),
        mq::KeyCode::World1 => None,
        mq::KeyCode::World2 => None,
        mq::KeyCode::KpAdd => Some(KeyCode::NumpadAdd),
        mq::KeyCode::Apostrophe => Some(KeyCode::Quote),
        mq::KeyCode::Backslash => Some(KeyCode::Backslash),
        mq::KeyCode::Comma => Some(KeyCode::Comma),
        mq::KeyCode::KpDecimal => Some(KeyCode::NumpadDecimal),
        mq::KeyCode::KpDivide => Some(KeyCode::NumpadDivide),
        mq::KeyCode::Equal => Some(KeyCode::Equal),
        mq::KeyCode::GraveAccent => Some(KeyCode::Backquote),
        mq::KeyCode::LeftAlt => Some(KeyCode::AltLeft),
        mq::KeyCode::LeftBracket => Some(KeyCode::BracketLeft),
        mq::KeyCode::LeftControl => Some(KeyCode::ControlLeft),
        mq::KeyCode::LeftShift => Some(KeyCode::ShiftLeft),
        mq::KeyCode::LeftSuper => Some(KeyCode::SuperLeft),
        mq::KeyCode::Minus => Some(KeyCode::Minus),
        mq::KeyCode::KpMultiply => Some(KeyCode::NumpadMultiply),
        mq::KeyCode::KpEnter => Some(KeyCode::NumpadEnter),
        mq::KeyCode::KpEqual => Some(KeyCode::NumpadEqual),
        mq::KeyCode::Period => Some(KeyCode::Period),
        mq::KeyCode::RightAlt => Some(KeyCode::AltRight),
        mq::KeyCode::RightBracket => Some(KeyCode::BracketRight),
        mq::KeyCode::RightControl => Some(KeyCode::ControlRight),
        mq::KeyCode::RightShift => Some(KeyCode::ShiftRight),
        mq::KeyCode::RightSuper => Some(KeyCode::SuperRight),
        mq::KeyCode::Semicolon => Some(KeyCode::Semicolon),
        mq::KeyCode::Slash => Some(KeyCode::Slash),
        mq::KeyCode::KpSubtract => Some(KeyCode::NumpadSubtract),
        mq::KeyCode::Tab => Some(KeyCode::Tab),
        mq::KeyCode::CapsLock => Some(KeyCode::CapsLock),
        mq::KeyCode::Unknown => None,
    }
}
