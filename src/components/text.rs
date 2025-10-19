use super::Component;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red, 
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    Gray,
    BrightRed, 
    Lime,
    Banana,
    LightBlue,
    Pink,
    LightCyan,
    BrightWhite,
    RGB(u8, u8, u8)
}

impl Color {
    
    pub fn ansi_fg(&self) -> String {
        match self {
            Color::Black => "\x1b[30m".to_string(),
            Color::Red => "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),

            Color::Gray => "\x1b[90m".to_string(),
            Color::BrightRed => "\x1b[91m".to_string(),
            Color::Lime => "\x1b[92m".to_string(),
            Color::Banana => "\x1b[93m".to_string(),
            Color::LightBlue => "\x1b[94m".to_string(),
            Color::Pink => "\x1b[95m".to_string(),
            Color::LightCyan => "\x1b[96m".to_string(),
            Color::BrightWhite => "\x1b[97m".to_string(),
            Color::RGB(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }
    
    
    pub fn ansi_bg(&self) -> String {
        match self {
            Color::Black => "\x1b[40m".to_string(),
            Color::Red => "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),

            Color::Gray => "\x1b[100m".to_string(),
            Color::BrightRed => "\x1b[101m".to_string(),
            Color::Lime => "\x1b[102m".to_string(),
            Color::Banana => "\x1b[103m".to_string(),
            Color::LightBlue => "\x1b[104m".to_string(),
            Color::Pink => "\x1b[105m".to_string(),
            Color::LightCyan => "\x1b[106m".to_string(),
            Color::BrightWhite => "\x1b[107m".to_string(),
            Color::RGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Style {
    Bold,
    Dim,
    Italic,
    Underlined,
    Blink,
    ReverseVideo,
    Concealed,
    Strikethrough
}

impl Style {
    
    pub fn ansi(&self) -> String {
        match self {
            Style::Bold => "\x1b[1m",
            Style::Dim => "\x1b[2m",
            Style::Italic => "\x1b[3m",
            Style::Underlined => "\x1b[4m",
            Style::Blink => "\x1b[5m",
            Style::ReverseVideo => "\x1b[7m",
            Style::Concealed => "\x1b[8m",
            Style::Strikethrough => "\x1b[9m"
        }.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub(crate) spans: Vec<TextSpan>
}

impl Text {
    
    pub fn new() -> Self {
        Self {
            spans: Vec::new()
        }
    }

    
    pub fn span(&mut self, content: &str) -> &mut TextSpan {
        self.spans.push(TextSpan::new(content));
        self.spans.last_mut().unwrap()
    }

    
    pub fn span_from(&mut self, span: TextSpan) {
        self.spans.push(span)
    }

    
    pub fn bold_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.bold();
        })
    }

    
    pub fn dim_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.dim();
        })
    }

    
    pub fn italic_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.italic();
        })
    }

    
    pub fn underline_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.underlined();
        })
    }

    
    pub fn blink_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.blink();
        })
    }

    
    pub fn reverse_video_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.reverse_video();
        })
    }

    
    pub fn conceal_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.concealed();
        })
    }

    
    pub fn strikethrough_all(&mut self) {
        self.spans.iter_mut().for_each(|s| {
            s.strikethrough();
        })
    }

    
    pub fn render_plain(&self) -> String {
        let mut output = String::new();

        for span in &self.spans {
            output.push_str(&span.render_plain())
        }

        output
    }
}

impl Component for Text {
    
    fn render(&self) -> String {
        let mut output = String::new();

        for span in &self.spans {
            output.push_str(&span.render())
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct TextSpan {
    content: String,
    color: Option<Color>,
    bg_color: Option<Color>,
    styles: Vec<Style>
}

impl TextSpan {
    
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            color: None,
            bg_color: None,
            styles: Vec::new(),
        }
    }

    
    pub fn contents(&mut self, contents: &str) -> &mut Self {
        self.content = contents.to_string();
        self
    }

    
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }
    
    
    pub fn bg_color(&mut self, color: Color) -> &mut Self {
        self.bg_color = Some(color);
        self
    }

    
    pub fn bold(&mut self) -> &mut Self {
        self.styles.push(Style::Bold);
        self
    }
    
    pub fn dim(&mut self) -> &mut Self {
        self.styles.push(Style::Dim);
        self
    }
    
    pub fn italic(&mut self) -> &mut Self {
        self.styles.push(Style::Italic);
        self
    }
    
    pub fn underlined(&mut self) -> &mut Self {
        self.styles.push(Style::Underlined);
        self
    }
    
    pub fn blink(&mut self) -> &mut Self {
        self.styles.push(Style::Blink);
        self
    }
    
    pub fn reverse_video(&mut self) -> &mut Self {
        self.styles.push(Style::ReverseVideo);
        self
    }
    
    pub fn concealed(&mut self) -> &mut Self {
        self.styles.push(Style::Concealed);
        self
    }
    
    pub fn strikethrough(&mut self) -> &mut Self {
        self.styles.push(Style::Strikethrough);
        self
    }

    pub fn render_plain(&self) -> String {
        self.content.clone()
    }
}

impl Component for TextSpan {
    
    fn render(&self) -> String {
        let mut output = String::new();
        
        for style in &self.styles {
            output.push_str(&style.ansi())
        }
        if let Some(color) = self.color {
            output.push_str(&color.ansi_fg());
        }
        if let Some(bg) = self.bg_color {
            output.push_str(&bg.ansi_bg());
        }
        
        output.push_str(&self.content);
        output.push_str("\x1b[0m");
        output
    }
}

impl From<&str> for Text {
    
    fn from(s: &str) -> Self {
        let mut text = Text::new();
        text.span(s);
        text
    }
}

impl From<String> for Text {
    
    fn from(s: String) -> Self {
        let mut text = Text::new();
        text.span(&s);
        text
    }
}

impl From<TextSpan> for Text {
    
    fn from(span: TextSpan) -> Self {
        let mut text = Text::new();
        text.span_from(span);
        text
    }
}