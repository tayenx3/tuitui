use super::Component;
use super::Text;

/// * Separator - a horizontal seperator that repeats the string it was handed
/// 
/// ```no_run
/// ui.separator("*+", 10)
/// ```
/// 
/// Output:
/// ```plain_text
/// *+*+*+*+*+*+*+*+*+*+
/// ```
pub struct Separator {
    text: Text,
    repeat: usize
}

impl Separator {
    
    pub fn new(text: Text, repeat: usize) -> Self {
        Self {
            text: text,
            repeat
        }
    }
}

impl Component for Separator {
    
    fn render(&self) -> String {
        self.text.render().repeat(self.repeat)
    }
}