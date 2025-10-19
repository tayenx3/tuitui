pub mod separator;
pub mod widget;
pub mod ascii_art;
pub mod text;

pub use separator::Separator;
pub use widget::*;
pub use ascii_art::AsciiArt;
pub use text::Text;

/// * Defines the way an object is rendered
/// 
/// You can easily `impl` this with your structs:
/// ```no_run
/// struct MyComponent;
/// 
/// impl Component for MyComponent {
///     fn render(&self) -> String {
///         "This is a very cool component".to_string()
///     }
/// }
/// ```
pub trait Component {
    fn render(&self) -> String;
}
