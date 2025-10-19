use crossterm::style::{style, Stylize};
use figlet_rs::FIGfont;

pub trait Component {
    fn render(&self) -> String;
}

pub struct Heading {
    text: String
}

impl Heading {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string()
        }
    }
}


impl Component for Heading {
    fn render(&self) -> String {
        style(self.text.clone()).bold().to_string()
    }
}


pub struct Paragraph {
    text: String
}

impl Paragraph {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string()
        }
    }
}

impl Component for Paragraph {
    fn render(&self) -> String {
        self.text.clone()
    }
}


pub struct Separator {
    text: String
}

impl Separator {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string()
        }
    }
}

impl Component for Separator {
    fn render(&self) -> String {
        self.text.repeat(10)
    }
}

#[derive(Debug, Clone)]
pub struct Widget {
    pub style: WidgetStyle,
    pub width: u16,
    pub height: u16,
    pub contents: Option<String>
}

impl Component for Widget {
    fn render(&self) -> String {
        let lines = if let Some(s) = &self.contents {
            Some(s.lines().collect::<Vec<_>>())
        } else {
            None
        };

        let width = if let Some(ref lines) = lines {
            lines.iter().max_by_key(|c| c.len()).unwrap().len().min(self.width.into())
        } else {
            self.width.into()
        };

        let mut buffer = String::new();
        buffer.push(self.style.top_left);
        for _ in 0..width {
            buffer.push(self.style.top_horizontal);
        }
        buffer.push(self.style.top_right);
        buffer.push('\n');

        for l in 0..(self.height - 2) {
            buffer.push(self.style.left_vertical);
            for i in 0..width {
                if let Some(ref lines) = lines {
                    let binding = lines
                        .get(l as usize)
                        .unwrap_or(&" ")
                        .chars()
                        .collect::<Vec<_>>()
                    ;
                    let line = binding
                        .get(i as usize)
                        .unwrap_or(&' ')
                    ;
                    buffer.push(*line)
                } else {
                    buffer.push(' ');
                }
            }
            buffer.push(self.style.right_vertical);
            buffer.push('\n');
        }
        
        buffer.push(self.style.bottom_left);
        for _ in 0..width {
            buffer.push(self.style.bottom_horizontal);
        }
        buffer.push(self.style.bottom_right);
        
        buffer
    }
}

impl Widget {
    pub fn new() -> WidgetBuilder {
        WidgetBuilder::new()
    }
}

#[derive(Debug, Clone)]
pub struct WidgetStyle {
    // Borders
    pub top_left: char,
    pub top_right: char, 
    pub bottom_left: char,
    pub bottom_right: char,
    pub top_horizontal: char,
    pub left_vertical: char,
    pub bottom_horizontal: char,
    pub right_vertical: char,

    // Later... colors?
}

impl WidgetStyle {
    pub fn tuitui_classic() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└', 
            bottom_right: '┘',
            top_horizontal: '─',
            left_vertical: '│',
            bottom_horizontal: '─',
            right_vertical: '│',
        }
    }

    pub fn tuitui_heavy_box() -> Self {
        Self {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗', 
            bottom_right: '┛',
            top_horizontal: '━',
            left_vertical: '┃',
            bottom_horizontal: '━',
            right_vertical: '┃',
        }
    }
    pub fn tuitui_rounded() -> Self {
        Self {
            top_left: '╭', top_right: '╮',
            bottom_left: '╰', bottom_right: '╯',
            top_horizontal: '─', left_vertical: '│',
            bottom_horizontal: '─', right_vertical: '│',
        }
    }
    
    pub fn tuitui_double() -> Self {
        Self {
            top_left: '╔', top_right: '╗',
            bottom_left: '╚', bottom_right: '╝', 
            top_horizontal: '═', left_vertical: '║',
            bottom_horizontal: '═', right_vertical: '║',
        }
    }
    
    pub fn tuitui_ascii() -> Self {
        Self {
            top_left: '+', top_right: '+',
            bottom_left: '+', bottom_right: '+',
            top_horizontal: '-', left_vertical: '|',
            bottom_horizontal: '-', right_vertical: '|',
        }
    }

    pub fn custom(
        top_left: char,
        top_right: char, 
        bottom_left: char,
        bottom_right: char,
        top_horizontal: char,
        left_vertical: char,
        bottom_horizontal: char,
        right_vertical: char
    ) -> Self {
        Self {
            top_left, top_right, bottom_left, bottom_right, top_horizontal, left_vertical, bottom_horizontal, right_vertical
        }
    }
    
    pub fn retro_ibm() -> Self {
        Self {
            top_left: '▄', top_right: '▄',
            bottom_left: '█', bottom_right: '█',
            top_horizontal: '▄', left_vertical: '█',
            bottom_horizontal: '▄', right_vertical: '█',
        }
    }
    
    pub fn retro_apple2() -> Self {
        Self {
            top_left: '█', top_right: '█',
            bottom_left: '█', bottom_right: '█',
            top_horizontal: '█', left_vertical: '█',
            bottom_horizontal: '█', right_vertical: '█',
        }
    }
    
    pub fn retro_c64() -> Self {
        Self {
            top_left: '▛', top_right: '▜',
            bottom_left: '▙', bottom_right: '▟',
            top_horizontal: '▀', left_vertical: '▌',
            bottom_horizontal: '▄', right_vertical: '▐',
        }
    }

    pub fn no_border() -> Self {
        Self {
            top_left: ' ', top_right: ' ',
            bottom_left: ' ', bottom_right: ' ',
            top_horizontal: ' ', left_vertical: ' ',
            bottom_horizontal: ' ', right_vertical: ' ',
        }
    }

    pub fn dotted() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            top_horizontal: '┄',
            left_vertical: '┊',
            bottom_horizontal: '┄',
            right_vertical: '┊',
        }
    }

    pub fn dashed() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            top_horizontal: '╌',
            left_vertical: '╎',
            bottom_horizontal: '╌',
            right_vertical: '╎',
        }
    }

    pub fn from_name(name: &str) -> Self {
        let n = name.to_lowercase();
        match n.strip_prefix("tuitui ").unwrap_or(&n) {
            "classic" | "default" => Self::tuitui_classic(),
            "heavy box" | "thick box" | "bold box" => Self::tuitui_heavy_box(),
            "rounded" | "smooth" => Self::tuitui_rounded(),
            "double" => Self::tuitui_double(),
            "ascii" | "basic" => Self::tuitui_ascii(),
            "retro" | "retro ibm" | "ibm" => Self::retro_ibm(),
            "retro apple2" | "retro apple 2" | "apple" | "retro apple" | "apple2" | "apple 2" => Self::retro_apple2(),
            "retro c64" | "commodore" | "c64" => Self::retro_c64(),
            "none" | "no border" | "no-border" => Self::no_border(),
            "dotted" | "dot" => Self::dotted(),
            "dashed" | "dash" => Self::dashed(),
            _ => Self::tuitui_classic()
        }
    }
}

pub struct WidgetBuilder {
    widget: Widget
}

impl WidgetBuilder {
    fn new() -> Self {
        Self {
            widget: Widget {
                style: WidgetStyle::from_name("Tuitui Classic"),
                width: 10,
                height: 10,
                contents: None
            }
        }
    }

    pub fn with_width(mut self, w: u16) -> Self {
        self.widget.width = w.max(2); self
    }

    pub fn with_height(mut self, h: u16) -> Self {
        self.widget.height = h.max(2); self
    }

    pub fn with_style(mut self, style: WidgetStyle) -> Self {
        self.widget.style = style; self
    }

    pub fn with_contents(mut self, contents: &str) -> Self {
        self.widget.contents = Some(contents.to_string());

        let lines = contents.lines().collect::<Vec<_>>();
        let largest = lines.iter().max_by_key(|s| s.len()).unwrap_or(&"").len();
        if largest > self.widget.width as usize - 2 {
            self.widget.width = contents.len() as u16 + 2;
        }

        if lines.len() > self.widget.height as usize - 2 {
            self.widget.width = contents.len() as u16 + 2;
        }
        self
    }

    pub fn build(self) -> Widget {
        self.widget
    }
}

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