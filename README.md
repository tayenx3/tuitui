# tuitui 🐧

Build terminal interfaces faster. A batteries-included TUI framework for Rust 
with low boilerplate and a rich component ecosystem.

## Why tuitui?

- **Low boilerplate** - Focus on your app, not framework code
- **Batteries included** - ASCII art, widgets, and more built-in
- **Fluent API** - Intuitive method chaining
- **Composable** - Easy to build custom components

## Quick Start

```rust
use tuitui::prelude::*;

struct Mascot;

impl App for Mascot {
    fn display(&mut self, ui: &mut Ui) {
        ui
            .ascii_art("Tuitui")
            .text(text!(
                span!("   _\n", color Color::RGB(122, 98, 168)),
                span!(" (", color Color::RGB(122, 98, 168)),
                span!("'v'", color Color::RGB(188, 163, 225)), 
                span!(")\n", color Color::RGB(122, 98, 168)),
                span!("//", color Color::RGB(122, 98, 168)),
                span!("-=-", color Color::RGB(245, 235, 220)),
                span!("\\\\\n", color Color::RGB(122, 98, 168)),
                span!("(\\", color Color::RGB(122, 98, 168)),
                span!("_=_", color Color::RGB(245, 235, 220)),
                span!("/)\n", color Color::RGB(122, 98, 168)),
                span!(" ^^ ^^", color Color::RGB(122, 98, 168))
            ));
    }
    
    fn update(&mut self, key: Option<Key>) -> bool {
        false
    }

    fn get_fps(&self) -> u32 {
        60
    }
}

fn main() -> Result<()> {
    AppRuntime::new().run(Mascot)
}
```

> Don't be scared! This is a full setup so it might look big!

[Join our flock](https://discord.gg/QQ66EQ5qa3), contribute, and build **amazing-ness** 🐧