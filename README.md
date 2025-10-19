# tuitui 🐧

Build terminal interfaces faster. A batteries-included TUI framework for Rust 
with low boilerplate and a rich component ecosystem.

## Why tuitui?

- **Friendly API** - Gentle learning curve
- **Low Boilerplate** - Focus on your app, not framework code
- **Batteries Included** - Forms, tables, lists, and more built-in
- **Composable** - Easy to build and share custom components
- **Predictable** - Follows Rust conventions, no hidden magic

## Quick Start

```rust
use std::collections::VecDeque;
use tuitui::prelude::*;

struct NewKeyLoggerApp {
    keys: VecDeque<Key>,
    quit: bool
}

impl NewKeyLoggerApp {
    fn new() -> Self {
        Self { keys: VecDeque::new(), quit: false }
    }
}

impl App for NewKeyLoggerApp {
    fn display(&mut self, ui: &mut Ui) {
        ui
            .ascii_art("Tuitui Key Logger")
            .separator("*=")
            .paragraph("This a Tuitui Key Logger")
            .paragraph("Press Esc to quit")
            .widget(&format!("Last 15 Keys:\n{}", 
                self.keys
                .iter()
                .map(|k| 
                    format!(
                        " - {}", k.to_friendly_string()
                    )
                )
                .collect::<Vec<_>>()
                .join("\n")
            ))
        ;
    }
    
    fn update(&mut self, key: Option<Key>) -> bool {
        if let None = key {
            return true
        }

        let key = key.unwrap();
        match key.key {
            InputKey::Esc => self.quit = true,
            _ => {
                self.keys.push_back(key);
                if self.keys.len() > 15 {
                    self.keys.pop_front();
                }
            }
        }

        !self.quit
    }

    fn get_fps(&self) -> u32 {
        60
    }
}

fn main() -> Result<()> {
    AppRuntime::new().run(NewKeyLoggerApp::new())
}
```

> Don't be scared! This is a full setup so it might look big!

Join our flock, contribute, and build amazing-ness