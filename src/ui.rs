use crate::components::*;

/// * Builds your user interface with a fluent API.
///
/// Add components in the order they should appear:
/// ```no_run
/// let mut ui = Ui::new();
/// ui
///  .heading("Welcome")
///  .paragraph("This is my app")
///  .widget(some_component);
/// ```
pub struct Ui {
    pub(crate) components: Vec<Box<dyn Component>>
}

impl Ui {
    
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }
    
    
    pub fn text(&mut self, content: Text) -> &mut Self {
        self.components.push(Box::new(content));
        self
    }
    
    
    pub fn separator(&mut self, pattern: Text, repeat: usize) -> &mut Self {
        let text: Text = pattern.into();
        self.components.push(Box::new(Separator::new(text, repeat)));
        self
    }
    
    
    pub fn paragraph(&mut self, content: Text) -> &mut Self {
        self.text(content)
    }
    
    
    pub fn heading(&mut self, content: Text) -> &mut Self {
        let mut text: Text = content.into();
        text.bold_all();
        self.components.push(Box::new(text));
        self
    }

    
    pub fn widget<F>(&mut self, build: F) -> &mut Self
    where
        F: FnOnce(WidgetBuilder) -> Widget
    {
        let builder = Widget::new();
        self.components.push(Box::new(build(builder)));
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