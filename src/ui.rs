use crate::components::*;

pub struct Ui {
    pub(crate) components: Vec<Box<dyn Component>>
}

impl Ui {
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }
    
    pub fn heading(&mut self, heading: &str) -> &mut Self {
        self.components.push(Box::new(Heading::new(heading)));
        self
    }

    pub fn paragraph(&mut self, paragraph: &str) -> &mut Self {
        self.components.push(Box::new(Paragraph::new(paragraph)));
        self
    }

    pub fn separator(&mut self, separator: &str) -> &mut Self {
        self.components.push(Box::new(Separator::new(separator)));
        self
    }

    pub fn widget(&mut self, contents: &str) -> &mut Self {
        self.components.push(
            Box::new(
                Widget::new()
                    .with_style(WidgetStyle::from_name("Tuitui Rounded"))
                    .with_height(17)
                    .with_contents(contents)
                    .build()
            )
        );
        self
    }

    pub fn ascii_art(&mut self, text: &str) -> &mut Self {
        self.components.push(Box::new(AsciiArt::new(text)));
        self
    }

    pub fn render(&self) -> String {
        let mut buffer = String::new();
        
        for component in &self.components {
            buffer.push_str(&component.render());
            buffer.push('\n');
        }
        
        buffer
    }
    
    pub fn clear(&mut self) {
        self.components.clear();
    }
}