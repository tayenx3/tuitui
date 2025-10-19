//! # tuitui 🐧
//! 
//! A friendly TUI framework for building terminal interfaces faster.
//! 
//! ## Features
//! 
//! - **Low boilerplate** - Focus on your app, not framework code
//! - **Batteries included** - ASCII art, widgets, and more built-in
//! - **Fluent API** - Intuitive method chaining
//! - **Composable** - Easy to build custom components
//! 
//! ## Quick Start
//! 
//! ```no_run
//! use tuitui::prelude::*;
//! 
//! struct MyApp;
//! 
//! impl App for MyApp {
//!     fn display(&mut self, ui: &mut Ui) {
//!         ui.heading("Hello tuitui! 🐧");
//!     }
//! 
//!     fn update(&mut self) -> bool {
//!         false    
//!     }
//! 
//!     fn get_fps(&self) -> u32 {
//!         30    
//!     }
//! }
//! 
//! fn main() {
//!     AppRuntime::new().run(MyApp);
//! }
//! ```


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