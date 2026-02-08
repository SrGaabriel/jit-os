#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
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
    Backtick,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equals,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftBracket,
    RightBracket,
    Backslash,
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Quote,
    Enter,
    LeftShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    RightShift,
    LeftCtrl,
    LeftAlt,
    Space,
    RightAlt,
    RightCtrl,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    NumLock,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

impl KeyEvent {
    pub fn key(&self) -> Key {
        match self {
            KeyEvent::Pressed(key) | KeyEvent::Released(key) => *key,
        }
    }

    pub fn is_pressed(&self) -> bool {
        matches!(self, KeyEvent::Pressed(_))
    }

    pub fn is_released(&self) -> bool {
        matches!(self, KeyEvent::Released(_))
    }

    pub fn from_scancode(scancode: u8) -> Self {
        if scancode & 0x80 != 0 {
            KeyEvent::Released(Key::from_scancode(scancode & 0x7F))
        } else {
            KeyEvent::Pressed(Key::from_scancode(scancode))
        }
    }
}

impl Key {
    pub fn to_ascii(&self, shift: bool) -> Option<u8> {
        match self {
            Key::A => Some(if shift { b'A' } else { b'a' }),
            Key::B => Some(if shift { b'B' } else { b'b' }),
            Key::C => Some(if shift { b'C' } else { b'c' }),
            Key::D => Some(if shift { b'D' } else { b'd' }),
            Key::E => Some(if shift { b'E' } else { b'e' }),
            Key::F => Some(if shift { b'F' } else { b'f' }),
            Key::G => Some(if shift { b'G' } else { b'g' }),
            Key::H => Some(if shift { b'H' } else { b'h' }),
            Key::I => Some(if shift { b'I' } else { b'i' }),
            Key::J => Some(if shift { b'J' } else { b'j' }),
            Key::K => Some(if shift { b'K' } else { b'k' }),
            Key::L => Some(if shift { b'L' } else { b'l' }),
            Key::M => Some(if shift { b'M' } else { b'm' }),
            Key::N => Some(if shift { b'N' } else { b'n' }),
            Key::O => Some(if shift { b'O' } else { b'o' }),
            Key::P => Some(if shift { b'P' } else { b'p' }),
            Key::Q => Some(if shift { b'Q' } else { b'q' }),
            Key::R => Some(if shift { b'R' } else { b'r' }),
            Key::S => Some(if shift { b'S' } else { b's' }),
            Key::T => Some(if shift { b'T' } else { b't' }),
            Key::U => Some(if shift { b'U' } else { b'u' }),
            Key::V => Some(if shift { b'V' } else { b'v' }),
            Key::W => Some(if shift { b'W' } else { b'w' }),
            Key::X => Some(if shift { b'X' } else { b'x' }),
            Key::Y => Some(if shift { b'Y' } else { b'y' }),
            Key::Z => Some(if shift { b'Z' } else { b'z' }),
            Key::Num1 => Some(if shift { b'!' } else { b'1' }),
            Key::Num2 => Some(if shift { b'@' } else { b'2' }),
            Key::Num3 => Some(if shift { b'#' } else { b'3' }),
            Key::Num4 => Some(if shift { b'$' } else { b'4' }),
            Key::Num5 => Some(if shift { b'%' } else { b'5' }),
            Key::Num6 => Some(if shift { b'^' } else { b'6' }),
            Key::Num7 => Some(if shift { b'&' } else { b'7' }),
            Key::Num8 => Some(if shift { b'*' } else { b'8' }),
            Key::Num9 => Some(if shift { b'(' } else { b'9' }),
            Key::Num0 => Some(if shift { b')' } else { b'0' }),
            Key::Minus => Some(if shift { b'_' } else { b'-' }),
            Key::Equals => Some(if shift { b'+' } else { b'=' }),
            Key::LeftBracket => Some(if shift { b'{' } else { b'[' }),
            Key::RightBracket => Some(if shift { b'}' } else { b']' }),
            Key::Backslash => Some(if shift { b'|' } else { b'\\' }),
            Key::Semicolon => Some(if shift { b':' } else { b';' }),
            Key::Quote => Some(if shift { b'"' } else { b'\'' }),
            Key::Backtick => Some(if shift { b'~' } else { b'`' }),
            Key::Comma => Some(if shift { b'<' } else { b',' }),
            Key::Dot => Some(if shift { b'>' } else { b'.' }),
            Key::Slash => Some(if shift { b'?' } else { b'/' }),
            Key::Space => Some(b' '),
            Key::Enter => Some(b'\n'),
            Key::Tab => Some(b'\t'),
            Key::Backspace => Some(0x08),
            _ => None,
        }
    }

    fn from_scancode(code: u8) -> Key {
        match code {
            0x01 => Key::Escape,
            0x02 => Key::Num1,
            0x03 => Key::Num2,
            0x04 => Key::Num3,
            0x05 => Key::Num4,
            0x06 => Key::Num5,
            0x07 => Key::Num6,
            0x08 => Key::Num7,
            0x09 => Key::Num8,
            0x0A => Key::Num9,
            0x0B => Key::Num0,
            0x0C => Key::Minus,
            0x0D => Key::Equals,
            0x0E => Key::Backspace,
            0x0F => Key::Tab,
            0x10 => Key::Q,
            0x11 => Key::W,
            0x12 => Key::E,
            0x13 => Key::R,
            0x14 => Key::T,
            0x15 => Key::Y,
            0x16 => Key::U,
            0x17 => Key::I,
            0x18 => Key::O,
            0x19 => Key::P,
            0x1A => Key::LeftBracket,
            0x1B => Key::RightBracket,
            0x1C => Key::Enter,
            0x1D => Key::LeftCtrl,
            0x1E => Key::A,
            0x1F => Key::S,
            0x20 => Key::D,
            0x21 => Key::F,
            0x22 => Key::G,
            0x23 => Key::H,
            0x24 => Key::J,
            0x25 => Key::K,
            0x26 => Key::L,
            0x27 => Key::Semicolon,
            0x28 => Key::Quote,
            0x29 => Key::Backtick,
            0x2A => Key::LeftShift,
            0x2B => Key::Backslash,
            0x2C => Key::Z,
            0x2D => Key::X,
            0x2E => Key::C,
            0x2F => Key::V,
            0x30 => Key::B,
            0x31 => Key::N,
            0x32 => Key::M,
            0x33 => Key::Comma,
            0x34 => Key::Dot,
            0x35 => Key::Slash,
            0x36 => Key::RightShift,
            0x38 => Key::LeftAlt,
            0x39 => Key::Space,
            0x3A => Key::CapsLock,
            0x3B => Key::F1,
            0x3C => Key::F2,
            0x3D => Key::F3,
            0x3E => Key::F4,
            0x3F => Key::F5,
            0x40 => Key::F6,
            0x41 => Key::F7,
            0x42 => Key::F8,
            0x43 => Key::F9,
            0x44 => Key::F10,
            0x45 => Key::NumLock,
            0x46 => Key::ScrollLock,
            0x47 => Key::Home,
            0x48 => Key::Up,
            0x49 => Key::PageUp,
            0x4B => Key::Left,
            0x4D => Key::Right,
            0x4F => Key::End,
            0x50 => Key::Down,
            0x51 => Key::PageDown,
            0x52 => Key::Insert,
            0x53 => Key::Delete,
            0x57 => Key::F11,
            0x58 => Key::F12,
            _ => Key::Unknown(code),
        }
    }

    pub fn to_scancode(&self) -> Option<u8> {
        match self {
            Key::Escape => Some(0x01),
            Key::Num1 => Some(0x02),
            Key::Num2 => Some(0x03),
            Key::Num3 => Some(0x04),
            Key::Num4 => Some(0x05),
            Key::Num5 => Some(0x06),
            Key::Num6 => Some(0x07),
            Key::Num7 => Some(0x08),
            Key::Num8 => Some(0x09),
            Key::Num9 => Some(0x0A),
            Key::Num0 => Some(0x0B),
            Key::Minus => Some(0x0C),
            Key::Equals => Some(0x0D),
            Key::Backspace => Some(0x0E),
            Key::Tab => Some(0x0F),
            Key::Q => Some(0x10),
            Key::W => Some(0x11),
            Key::E => Some(0x12),
            Key::R => Some(0x13),
            Key::T => Some(0x14),
            Key::Y => Some(0x15),
            Key::U => Some(0x16),
            Key::I => Some(0x17),
            Key::O => Some(0x18),
            Key::P => Some(0x19),
            Key::LeftBracket => Some(0x1A),
            Key::RightBracket => Some(0x1B),
            Key::Enter => Some(0x1C),
            Key::LeftCtrl => Some(0x1D),
            Key::A => Some(0x1E),
            Key::S => Some(0x1F),
            Key::D => Some(0x20),
            Key::F => Some(0x21),
            Key::G => Some(0x22),
            Key::H => Some(0x23),
            Key::J => Some(0x24),
            Key::K => Some(0x25),
            Key::L => Some(0x26),
            Key::Semicolon => Some(0x27),
            Key::Quote => Some(0x28),
            Key::Backtick => Some(0x29),
            Key::LeftShift => Some(0x2A),
            Key::Backslash => Some(0x2B),
            Key::Z => Some(0x2C),
            Key::X => Some(0x2D),
            Key::C => Some(0x2E),
            Key::V => Some(0x2F),
            Key::B => Some(0x30),
            Key::N => Some(0x31),
            Key::M => Some(0x32),
            Key::Comma => Some(0x33),
            Key::Dot => Some(0x34),
            Key::Slash => Some(0x35),
            Key::RightShift => Some(0x36),
            Key::LeftAlt => Some(0x38),
            Key::Space => Some(0x39),
            Key::CapsLock => Some(0x3A),
            Key::F1 => Some(0x3B),
            Key::F2 => Some(0x3C),
            Key::F3 => Some(0x3D),
            Key::F4 => Some(0x3E),
            Key::F5 => Some(0x3F),
            Key::F6 => Some(0x40),
            Key::F7 => Some(0x41),
            Key::F8 => Some(0x42),
            Key::F9 => Some(0x43),
            Key::F10 => Some(0x44),
            Key::NumLock => Some(0x45),
            Key::ScrollLock => Some(0x46),
            Key::Home => Some(0x47),
            Key::Up => Some(0x48),
            Key::PageUp => Some(0x49),
            Key::Left => Some(0x4B),
            Key::Right => Some(0x4D),
            Key::End => Some(0x4F),
            Key::Down => Some(0x50),
            Key::PageDown => Some(0x51),
            Key::Insert => Some(0x52),
            Key::Delete => Some(0x53),
            Key::F11 => Some(0x57),
            Key::F12 => Some(0x58),
            _ => None, // todo: actually implement them all
        }
    }
}

pub struct KeyboardState(u128);

impl KeyboardState {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn update(&mut self, scancode: u8) {
        let key = scancode & 0x7F;
        if scancode & 0x80 == 0 {
            self.press(key);
        } else {
            self.release(key);
        }
    }

    pub fn press(&mut self, scancode: u8) {
        self.0 |= 1u128 << scancode;
    }

    pub fn release(&mut self, scancode: u8) {
        self.0 &= !(1u128 << scancode);
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        if let Some(scancode) = key.to_scancode() {
            (self.0 & (1u128 << scancode)) != 0
        } else {
            false
        }
    }
}
