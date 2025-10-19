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
    
    pub fn heading(&mut self, heading: &str) -> &mut Self {
        self.components.push(Box::new(Heading::new(heading)));
        self
    }

    pub fn paragraph(&mut self, paragraph: &str) -> &mut Self {
        self.components.push(Box::new(Paragraph::new(paragraph)));
        self
    }

    pub fn separator(&mut self, separator: &str, repeat: usize) -> &mut Self {
        self.components.push(Box::new(Separator::new(separator, repeat)));
        self
    }

    pub fn widget(&mut self, contents: Option<&str>, widget_style: WidgetStyle) -> &mut Self {
        self.components.push(
            Box::new(
                if let Some(contents) = contents {
                    Widget::new()
                        .with_style(widget_style)
                        .with_height(17)
                        .with_contents(contents)
                        .build()
                } else {
                    Widget::new()
                        .with_style(widget_style)
                        .with_height(17)
                        .build()
                }
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