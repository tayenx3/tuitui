use strum_macros::{EnumIter, AsRefStr};
use convert_case::{Case, Casing};
use crossterm::event::{self, KeyEventKind};
use std::time::Duration;
use event::{KeyCode, KeyEvent, KeyModifiers};

const TYPEABLE_KEYS: &[InputKey] = &[
    InputKey::Q, InputKey::W, InputKey::E, InputKey::R, InputKey::T, InputKey::Y, InputKey::U, InputKey::I, InputKey::O, InputKey::P, 
    InputKey::A, InputKey::S, InputKey::D, InputKey::F, InputKey::G, InputKey::H, InputKey::J, InputKey::K, InputKey::L, 
    InputKey::Z, InputKey::X, InputKey::C, InputKey::V, InputKey::B, InputKey::N, InputKey::M,

    InputKey::Tilde, InputKey::Backtick, InputKey::Bang, InputKey::At, InputKey::Hashtag, InputKey::DollarSign, InputKey::Percent, 
    InputKey::Caret, InputKey::Ampersand, InputKey::Star, InputKey::ForwardSlash, InputKey::Backslash, InputKey::Pipe, InputKey::Minus, 
    InputKey::Underscore, InputKey::Plus, InputKey::Equal,

    InputKey::LeftParenthesis, InputKey::RightParenthesis, InputKey::LeftSquareBracket, InputKey::RightSquareBracket, 
    InputKey::LeftCurlyBracket, InputKey::RightCurlyBraket, InputKey::LeftAngleBracket, InputKey::RightAngleBracket,
];

const ACTION_KEYS: &[InputKey] = &[
    InputKey::Backspace, InputKey::Delete, InputKey::Enter, InputKey::Tab, InputKey::Esc,
    InputKey::UpArrow, InputKey::DownArrow, InputKey::LeftArrow, InputKey::RightArrow,
    InputKey::F1, InputKey::F2, InputKey::F3, InputKey::F4, InputKey::F5, InputKey::F6, InputKey::F7,
    InputKey::F8, InputKey::F9, InputKey::F10, InputKey::F11, InputKey::F12,
    InputKey::Insert, InputKey::End, InputKey::Home, InputKey::PrtSc
];

pub struct KeyConversionError {
    pub details: String
}


/// # Input Keys
/// 
/// * Contains all of the keys (except modifiers like Shift, Capslock, Alt, ...) that can be inputted on a laptop
/// * Keyboard only keys will be implemented in the future
#[derive(AsRefStr, EnumIter, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InputKey {
    // Letter keys
    Q, W, E, R, T, Y, U, I, O, P, A, S, D, F, G, H, J, K, L, Z, X, C, V, B, N, M,

    // Number keys
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,

     // Symbols
    Tilde, Backtick, Bang, At, Hashtag, DollarSign, Percent, Caret, Ampersand, Star,
    ForwardSlash, Backslash, Pipe, Minus, Underscore, Plus, Equal,

    // Delimiters
    LeftParenthesis, RightParenthesis, LeftSquareBracket, RightSquareBracket, 
    LeftCurlyBracket, RightCurlyBraket, LeftAngleBracket, RightAngleBracket,

    // Punctuation
    Comma, Dot, Semicolon, QuestionMark, Colon, Apostrophe, QuotationMark,

    // Other
    Space,

    // Action keys
    Backspace, Delete, Enter, Tab, BackTab, Esc, CapsLock,
    UpArrow, DownArrow, LeftArrow, RightArrow,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Insert, End, Home, PrtSc, PageUp, PageDown,
}

impl InputKey {
    pub fn to_debug_string(&self) -> String {
        self.as_ref().to_string()
    }

    pub fn to_friendly_string(&self) -> String {
        self.to_debug_string().to_case(Case::Title)
    }

    pub fn is_typeable_key(&self) -> bool {
        TYPEABLE_KEYS.contains(self)
    }

    pub fn is_action_key(&self) -> bool {
        ACTION_KEYS.contains(self)
    }
}

impl TryFrom<KeyEvent> for InputKey {
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let code = event.code;
        match code {
            KeyCode::Char(c) => match c
                .to_lowercase()
                .to_string()
                .chars()
                .nth(0)
                .unwrap() 
            {
                'q' => Ok(InputKey::Q),
                'w' => Ok(InputKey::W),
                'e' => Ok(InputKey::E),
                'r' => Ok(InputKey::R),
                't' => Ok(InputKey::T),
                'y' => Ok(InputKey::Y),
                'u' => Ok(InputKey::U),
                'i' => Ok(InputKey::I),
                'o' => Ok(InputKey::O),
                'p' => Ok(InputKey::P),
                'a' => Ok(InputKey::A),
                's' => Ok(InputKey::S),
                'd' => Ok(InputKey::D),
                'f' => Ok(InputKey::F),
                'g' => Ok(InputKey::G),
                'h' => Ok(InputKey::H),
                'j' => Ok(InputKey::J),
                'k' => Ok(InputKey::K),
                'l' => Ok(InputKey::L),
                'z' => Ok(InputKey::Z),
                'x' => Ok(InputKey::X),
                'c' => Ok(InputKey::C),
                'v' => Ok(InputKey::V),
                'b' => Ok(InputKey::B),
                'n' => Ok(InputKey::N),
                'm' => Ok(InputKey::M),
                ' ' => Ok(InputKey::Space),

                '~' => Ok(InputKey::Tilde),
                '`' => Ok(InputKey::Backtick),
                '!' => Ok(InputKey::Bang),
                '@' => Ok(InputKey::At),
                '#' => Ok(InputKey::Hashtag),
                '$' => Ok(InputKey::DollarSign),
                '%' => Ok(InputKey::Percent),
                '^' => Ok(InputKey::Caret),
                '&' => Ok(InputKey::Ampersand),
                '*' => Ok(InputKey::Star),
                '/' => Ok(InputKey::ForwardSlash),
                '\\' => Ok(InputKey::Backslash),
                '|' => Ok(InputKey::Pipe),
                '-' => Ok(InputKey::Minus),
                '_' => Ok(InputKey::Underscore),
                '+' => Ok(InputKey::Plus),
                '=' => Ok(InputKey::Equal),
                '(' => Ok(InputKey::LeftParenthesis),
                ')' => Ok(InputKey::RightParenthesis),
                '[' => Ok(InputKey::LeftSquareBracket),
                ']' => Ok(InputKey::RightSquareBracket),
                '{' => Ok(InputKey::LeftCurlyBracket),
                '}' => Ok(InputKey::RightCurlyBraket),
                '<' => Ok(InputKey::LeftAngleBracket),
                '>' => Ok(InputKey::RightAngleBracket),
                ',' => Ok(InputKey::Comma),
                '.' => Ok(InputKey::Dot),
                ';' => Ok(InputKey::Semicolon),
                '?' => Ok(InputKey::QuestionMark),
                ':' => Ok(InputKey::Colon),
                '\'' => Ok(InputKey::Apostrophe),
                '"' => Ok(InputKey::QuotationMark),
                
                '0' => Ok(InputKey::Num0),
                '1' => Ok(InputKey::Num1),
                '2' => Ok(InputKey::Num2),
                '3' => Ok(InputKey::Num3),
                '4' => Ok(InputKey::Num4),
                '5' => Ok(InputKey::Num5),
                '6' => Ok(InputKey::Num6),
                '7' => Ok(InputKey::Num7),
                '8' => Ok(InputKey::Num8),
                '9' => Ok(InputKey::Num9),
                _ => unreachable!()
            }

            KeyCode::Backspace => Ok(InputKey::Backspace),
            KeyCode::PrintScreen => Ok(InputKey::PrtSc),
            KeyCode::F(1) => Ok(InputKey::F1),
            KeyCode::F(2) => Ok(InputKey::F2),
            KeyCode::F(3) => Ok(InputKey::F3),
            KeyCode::F(4) => Ok(InputKey::F4),
            KeyCode::F(5) => Ok(InputKey::F5),
            KeyCode::F(6) => Ok(InputKey::F6),
            KeyCode::F(7) => Ok(InputKey::F7),
            KeyCode::F(8) => Ok(InputKey::F8),
            KeyCode::F(9) => Ok(InputKey::F9),
            KeyCode::F(10) => Ok(InputKey::F10),
            KeyCode::F(11) => Ok(InputKey::F11),
            KeyCode::F(12) => Ok(InputKey::F12),
            KeyCode::Delete => Ok(InputKey::Delete),
            KeyCode::End => Ok(InputKey::End),
            KeyCode::Insert => Ok(InputKey::Insert),
            KeyCode::Esc => Ok(InputKey::Esc),
            KeyCode::CapsLock => Ok(InputKey::CapsLock),
            KeyCode::BackTab => Ok(InputKey::BackTab),
            KeyCode::PageUp => Ok(InputKey::PageUp),
            KeyCode::PageDown => Ok(InputKey::PageDown),
            KeyCode::Up => Ok(InputKey::UpArrow),
            KeyCode::Down => Ok(InputKey::DownArrow),
            KeyCode::Left => Ok(InputKey::LeftArrow),
            KeyCode::Right => Ok(InputKey::RightArrow),
            KeyCode::Enter => Ok(InputKey::Enter),
            KeyCode::Home => Ok(InputKey::Home),
            KeyCode::Tab => Ok(InputKey::Tab),
            _ => Err(KeyConversionError { details: format!("Key not implemented: {}", code) })
        }
    }
    
    type Error = KeyConversionError;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Key {
    pub key: InputKey,
    pub modifiers: KeyModifiers,
    pub character: Option<char>,
}

impl Key {
    pub fn ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }
    
    pub fn alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }
    
    pub fn shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    pub fn to_friendly_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{} {}", self.key.to_friendly_string(), if let Some(ch) = self.character { format!("( {} )", ch)} else { "".to_string() }));
        output
    }
}

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }
    
    pub fn poll(&mut self, timeout: Duration) -> Option<Key> {
        if event::poll(timeout).ok()? {
            if let Ok(crossterm_event) = event::read() {
                if let event::Event::Key(key_event) = crossterm_event {
                    if !(key_event.kind == KeyEventKind::Press) {
                        return None
                    }
                    match InputKey::try_from(key_event) {
                        Ok(input_key) => {
                            let character = match key_event.code {
                                KeyCode::Char(c) => Some(c),
                                _ => None,
                            };
                            
                            return Some(Key {
                                key: input_key,
                                modifiers: key_event.modifiers,
                                character,
                            });
                        }
                        Err(e) => {
                            eprintln!("Unsupported key: {}", e.details);
                        }
                    }
                }
            }
        }
        None
    }
}