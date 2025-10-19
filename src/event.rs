use crate::inputs::Key;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Event {
    Key(Key),
    Quit,
    Resize(u16, u16),
}