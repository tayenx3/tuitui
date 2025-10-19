pub(crate) mod inputs;
pub(crate) mod event;
pub(crate) mod app;
pub(crate) mod ui;
pub(crate) mod components;

pub struct Terminal;

pub mod prelude {
    #![allow(unused_imports)]
    pub use crate::inputs::*;
    pub use crate::event::Event;
    pub use crate::app::*;
    pub use std::time::Duration;
    pub use crate::ui::*;
    pub use crate::components::*;
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn widget_test() {
        use super::prelude::*;

        let my_widget = Widget::new()
            .with_width(35)
            .with_height(5)
            .with_contents("Hello!")
            .with_style(WidgetStyle::from_name("dashed"))
            .build();

        eprintln!("{}", my_widget.render());
    }
}