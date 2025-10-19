use super::Component;
use figlet_rs::FIGfont;

pub struct AsciiArt {
    text: String
}

impl AsciiArt {
    
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string()
        }
    }
}

impl Component for AsciiArt {
    
    fn render(&self) -> String {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert(&self.text);
        figure.unwrap().to_string()
    }
}
