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
pub mod components;
pub mod macros;

pub mod prelude {
    #![allow(unused_imports)]
    pub use crate::inputs::*;
    pub use crate::event::Event;
    pub use crate::app::*;
    pub use std::time::Duration;
    pub use crate::ui::*;
    pub use crate::components::*;
    pub use crate::{span, text};
    pub use text::*;
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}