use super::Component;
use super::Text;
use strip_ansi_escapes::strip;

#[derive(Debug, Clone)]
pub struct Widget {
    pub style: WidgetStyle,
    pub width: u16,
    pub height: u16,
    pub contents: Text
}

impl Component for Widget {
    #[inline]
    fn render(&self) -> String {
        let binding = self.contents.render();
        let lines = binding.lines().collect::<Vec<_>>();

        let width = lines
            .iter()
            .max_by_key(|c| strip(c).len())
            .unwrap()
            .len()
            .min(self.width.into());

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
                let binding = lines
                    .get(l as usize)
                    .unwrap_or(&" ")
                ;
                let binding = 
                    String::from_utf8(
                        strip(binding)
                    )
                    .unwrap()
                    .chars()
                    .collect::<Vec<_>>();
                let c = binding
                    .get(i as usize)
                    .unwrap_or(&' ')
                ;
                buffer.push(*c)
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
    #[inline]
    pub fn new() -> WidgetBuilder {
        WidgetBuilder::new()
    }
}

#[derive(Debug, Clone)]
pub struct WidgetStyle {
    pub top_left: char,
    pub top_right: char, 
    pub bottom_left: char,
    pub bottom_right: char,
    pub top_horizontal: char,
    pub left_vertical: char,
    pub bottom_horizontal: char,
    pub right_vertical: char,
}

impl WidgetStyle {
    #[inline]
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

    #[inline]
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
    #[inline]
    pub fn tuitui_rounded() -> Self {
        Self {
            top_left: '╭', top_right: '╮',
            bottom_left: '╰', bottom_right: '╯',
            top_horizontal: '─', left_vertical: '│',
            bottom_horizontal: '─', right_vertical: '│',
        }
    }
    
    #[inline]
    pub fn tuitui_double() -> Self {
        Self {
            top_left: '╔', top_right: '╗',
            bottom_left: '╚', bottom_right: '╝', 
            top_horizontal: '═', left_vertical: '║',
            bottom_horizontal: '═', right_vertical: '║',
        }
    }
    
    #[inline]
    pub fn tuitui_ascii() -> Self {
        Self {
            top_left: '+', top_right: '+',
            bottom_left: '+', bottom_right: '+',
            top_horizontal: '-', left_vertical: '|',
            bottom_horizontal: '-', right_vertical: '|',
        }
    }

    #[inline]
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
    
    #[inline]
    pub fn retro_ibm() -> Self {
        Self {
            top_left: '▄', top_right: '▄',
            bottom_left: '█', bottom_right: '█',
            top_horizontal: '▄', left_vertical: '█',
            bottom_horizontal: '▄', right_vertical: '█',
        }
    }
    
    #[inline]
    pub fn retro_apple2() -> Self {
        Self {
            top_left: '█', top_right: '█',
            bottom_left: '█', bottom_right: '█',
            top_horizontal: '█', left_vertical: '█',
            bottom_horizontal: '█', right_vertical: '█',
        }
    }
    
    #[inline]
    pub fn retro_c64() -> Self {
        Self {
            top_left: '▛', top_right: '▜',
            bottom_left: '▙', bottom_right: '▟',
            top_horizontal: '▀', left_vertical: '▌',
            bottom_horizontal: '▄', right_vertical: '▐',
        }
    }

    #[inline]
    pub fn no_border() -> Self {
        Self {
            top_left: ' ', top_right: ' ',
            bottom_left: ' ', bottom_right: ' ',
            top_horizontal: ' ', left_vertical: ' ',
            bottom_horizontal: ' ', right_vertical: ' ',
        }
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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

#[derive(Debug, Clone)]
pub struct WidgetBuilder {
    widget: Widget
}

impl WidgetBuilder {
    #[inline]
    fn new() -> Self {
        Self {
            widget: Widget {
                style: WidgetStyle::from_name("Tuitui Classic"),
                width: 10,
                height: 10,
                contents: Text {
                    spans: Vec::new()
                }
            }
        }
    }

    #[inline]
    pub fn with_width(mut self, w: u16) -> Self {
        self.widget.width = w.max(2); self
    }

    #[inline]
    pub fn with_height(mut self, h: u16) -> Self {
        self.widget.height = h.max(2); self
    }

    #[inline]
    pub fn with_style(mut self, style: WidgetStyle) -> Self {
        self.widget.style = style; self
    }

    #[inline]
    pub fn with_contents(mut self, contents: Text) -> Self {
        self.widget.contents = contents.clone();

        let binding = contents.render();
        let lines = binding.lines().collect::<Vec<_>>();
        let largest = lines.iter().max_by_key(|s| s.len()).unwrap_or(&"").len();
        if largest > self.widget.width as usize - 2 {
            self.widget.width = contents.render().len() as u16 + 2;
        }

        if lines.len() > self.widget.height as usize - 2 {
            self.widget.width = contents.render().len() as u16 + 2;
        }
        self
    }

    #[inline]
    pub fn build(self) -> Widget {
        self.widget
    }
}