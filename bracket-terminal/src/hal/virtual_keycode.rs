//! Custom VirtualKeyCode enum that provides compatibility across different backends
//! and maintains stable key codes regardless of winit version changes.

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum VirtualKeyCode {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    NumpadAdd,
    Apostrophe,
    Apps,
    Asterisk,
    Plus,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    NumpadDecimal,
    NumpadDivide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    NumpadMultiply,
    Mute,
    MyComputer,
    NavigateForward,  // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    NumpadSubtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

impl VirtualKeyCode {
    /// Convert from winit 0.30's KeyCode to our VirtualKeyCode
    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    pub fn from_winit_keycode(keycode: winit::keyboard::KeyCode) -> Option<Self> {
        use winit::keyboard::KeyCode;

        match keycode {
            KeyCode::Digit1 => Some(VirtualKeyCode::Key1),
            KeyCode::Digit2 => Some(VirtualKeyCode::Key2),
            KeyCode::Digit3 => Some(VirtualKeyCode::Key3),
            KeyCode::Digit4 => Some(VirtualKeyCode::Key4),
            KeyCode::Digit5 => Some(VirtualKeyCode::Key5),
            KeyCode::Digit6 => Some(VirtualKeyCode::Key6),
            KeyCode::Digit7 => Some(VirtualKeyCode::Key7),
            KeyCode::Digit8 => Some(VirtualKeyCode::Key8),
            KeyCode::Digit9 => Some(VirtualKeyCode::Key9),
            KeyCode::Digit0 => Some(VirtualKeyCode::Key0),

            KeyCode::KeyA => Some(VirtualKeyCode::A),
            KeyCode::KeyB => Some(VirtualKeyCode::B),
            KeyCode::KeyC => Some(VirtualKeyCode::C),
            KeyCode::KeyD => Some(VirtualKeyCode::D),
            KeyCode::KeyE => Some(VirtualKeyCode::E),
            KeyCode::KeyF => Some(VirtualKeyCode::F),
            KeyCode::KeyG => Some(VirtualKeyCode::G),
            KeyCode::KeyH => Some(VirtualKeyCode::H),
            KeyCode::KeyI => Some(VirtualKeyCode::I),
            KeyCode::KeyJ => Some(VirtualKeyCode::J),
            KeyCode::KeyK => Some(VirtualKeyCode::K),
            KeyCode::KeyL => Some(VirtualKeyCode::L),
            KeyCode::KeyM => Some(VirtualKeyCode::M),
            KeyCode::KeyN => Some(VirtualKeyCode::N),
            KeyCode::KeyO => Some(VirtualKeyCode::O),
            KeyCode::KeyP => Some(VirtualKeyCode::P),
            KeyCode::KeyQ => Some(VirtualKeyCode::Q),
            KeyCode::KeyR => Some(VirtualKeyCode::R),
            KeyCode::KeyS => Some(VirtualKeyCode::S),
            KeyCode::KeyT => Some(VirtualKeyCode::T),
            KeyCode::KeyU => Some(VirtualKeyCode::U),
            KeyCode::KeyV => Some(VirtualKeyCode::V),
            KeyCode::KeyW => Some(VirtualKeyCode::W),
            KeyCode::KeyX => Some(VirtualKeyCode::X),
            KeyCode::KeyY => Some(VirtualKeyCode::Y),
            KeyCode::KeyZ => Some(VirtualKeyCode::Z),

            KeyCode::Escape => Some(VirtualKeyCode::Escape),

            KeyCode::F1 => Some(VirtualKeyCode::F1),
            KeyCode::F2 => Some(VirtualKeyCode::F2),
            KeyCode::F3 => Some(VirtualKeyCode::F3),
            KeyCode::F4 => Some(VirtualKeyCode::F4),
            KeyCode::F5 => Some(VirtualKeyCode::F5),
            KeyCode::F6 => Some(VirtualKeyCode::F6),
            KeyCode::F7 => Some(VirtualKeyCode::F7),
            KeyCode::F8 => Some(VirtualKeyCode::F8),
            KeyCode::F9 => Some(VirtualKeyCode::F9),
            KeyCode::F10 => Some(VirtualKeyCode::F10),
            KeyCode::F11 => Some(VirtualKeyCode::F11),
            KeyCode::F12 => Some(VirtualKeyCode::F12),
            KeyCode::F13 => Some(VirtualKeyCode::F13),
            KeyCode::F14 => Some(VirtualKeyCode::F14),
            KeyCode::F15 => Some(VirtualKeyCode::F15),
            KeyCode::F16 => Some(VirtualKeyCode::F16),
            KeyCode::F17 => Some(VirtualKeyCode::F17),
            KeyCode::F18 => Some(VirtualKeyCode::F18),
            KeyCode::F19 => Some(VirtualKeyCode::F19),
            KeyCode::F20 => Some(VirtualKeyCode::F20),
            KeyCode::F21 => Some(VirtualKeyCode::F21),
            KeyCode::F22 => Some(VirtualKeyCode::F22),
            KeyCode::F23 => Some(VirtualKeyCode::F23),
            KeyCode::F24 => Some(VirtualKeyCode::F24),

            KeyCode::PrintScreen => Some(VirtualKeyCode::Snapshot),
            KeyCode::ScrollLock => Some(VirtualKeyCode::Scroll),
            KeyCode::Pause => Some(VirtualKeyCode::Pause),

            KeyCode::Insert => Some(VirtualKeyCode::Insert),
            KeyCode::Home => Some(VirtualKeyCode::Home),
            KeyCode::Delete => Some(VirtualKeyCode::Delete),
            KeyCode::End => Some(VirtualKeyCode::End),
            KeyCode::PageDown => Some(VirtualKeyCode::PageDown),
            KeyCode::PageUp => Some(VirtualKeyCode::PageUp),

            KeyCode::ArrowLeft => Some(VirtualKeyCode::Left),
            KeyCode::ArrowUp => Some(VirtualKeyCode::Up),
            KeyCode::ArrowRight => Some(VirtualKeyCode::Right),
            KeyCode::ArrowDown => Some(VirtualKeyCode::Down),

            KeyCode::Backspace => Some(VirtualKeyCode::Back),
            KeyCode::Enter => Some(VirtualKeyCode::Return),
            KeyCode::Space => Some(VirtualKeyCode::Space),

            KeyCode::NumLock => Some(VirtualKeyCode::Numlock),
            KeyCode::Numpad0 => Some(VirtualKeyCode::Numpad0),
            KeyCode::Numpad1 => Some(VirtualKeyCode::Numpad1),
            KeyCode::Numpad2 => Some(VirtualKeyCode::Numpad2),
            KeyCode::Numpad3 => Some(VirtualKeyCode::Numpad3),
            KeyCode::Numpad4 => Some(VirtualKeyCode::Numpad4),
            KeyCode::Numpad5 => Some(VirtualKeyCode::Numpad5),
            KeyCode::Numpad6 => Some(VirtualKeyCode::Numpad6),
            KeyCode::Numpad7 => Some(VirtualKeyCode::Numpad7),
            KeyCode::Numpad8 => Some(VirtualKeyCode::Numpad8),
            KeyCode::Numpad9 => Some(VirtualKeyCode::Numpad9),

            KeyCode::NumpadAdd => Some(VirtualKeyCode::NumpadAdd),
            KeyCode::NumpadDecimal => Some(VirtualKeyCode::NumpadDecimal),
            KeyCode::NumpadDivide => Some(VirtualKeyCode::NumpadDivide),
            KeyCode::NumpadMultiply => Some(VirtualKeyCode::NumpadMultiply),
            KeyCode::NumpadSubtract => Some(VirtualKeyCode::NumpadSubtract),
            KeyCode::NumpadEnter => Some(VirtualKeyCode::NumpadEnter),
            KeyCode::NumpadEqual => Some(VirtualKeyCode::NumpadEquals),

            KeyCode::Quote => Some(VirtualKeyCode::Apostrophe),
            KeyCode::Backslash => Some(VirtualKeyCode::Backslash),
            KeyCode::BracketLeft => Some(VirtualKeyCode::LBracket),
            KeyCode::BracketRight => Some(VirtualKeyCode::RBracket),
            KeyCode::Comma => Some(VirtualKeyCode::Comma),
            KeyCode::Equal => Some(VirtualKeyCode::Equals),
            KeyCode::Backquote => Some(VirtualKeyCode::Grave),
            KeyCode::Minus => Some(VirtualKeyCode::Minus),
            KeyCode::Period => Some(VirtualKeyCode::Period),
            KeyCode::Semicolon => Some(VirtualKeyCode::Semicolon),
            KeyCode::Slash => Some(VirtualKeyCode::Slash),
            KeyCode::Tab => Some(VirtualKeyCode::Tab),

            KeyCode::AltLeft => Some(VirtualKeyCode::LAlt),
            KeyCode::AltRight => Some(VirtualKeyCode::RAlt),
            KeyCode::ControlLeft => Some(VirtualKeyCode::LControl),
            KeyCode::ControlRight => Some(VirtualKeyCode::RControl),
            KeyCode::ShiftLeft => Some(VirtualKeyCode::LShift),
            KeyCode::ShiftRight => Some(VirtualKeyCode::RShift),
            KeyCode::SuperLeft => Some(VirtualKeyCode::LWin),
            KeyCode::SuperRight => Some(VirtualKeyCode::RWin),

            KeyCode::ContextMenu => Some(VirtualKeyCode::Apps),
            KeyCode::CapsLock => Some(VirtualKeyCode::Capital),

            KeyCode::AudioVolumeDown => Some(VirtualKeyCode::VolumeDown),
            KeyCode::AudioVolumeUp => Some(VirtualKeyCode::VolumeUp),
            KeyCode::AudioVolumeMute => Some(VirtualKeyCode::Mute),

            KeyCode::MediaPlayPause => Some(VirtualKeyCode::PlayPause),
            KeyCode::MediaStop => Some(VirtualKeyCode::MediaStop),
            KeyCode::MediaTrackNext => Some(VirtualKeyCode::NextTrack),
            KeyCode::MediaTrackPrevious => Some(VirtualKeyCode::PrevTrack),

            KeyCode::Copy => Some(VirtualKeyCode::Copy),
            KeyCode::Paste => Some(VirtualKeyCode::Paste),
            KeyCode::Cut => Some(VirtualKeyCode::Cut),

            // Many KeyCode variants don't have direct equivalents
            _ => None,
        }
    }

    /// Convert to a scancode for compatibility with older APIs
    pub fn to_scancode(self) -> u32 {
        self as u32
    }
}
