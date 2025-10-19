use std::io::{self, Write};
use crate::{inputs::*, ui::Ui};

pub trait App {
    fn display(&mut self, ui: &mut Ui);
    fn update(&mut self, key: Option<Key>) -> bool;
    fn get_fps(&self) -> u32;
}

pub struct AppRuntime {
    input: InputHandler
}

impl AppRuntime {
    pub fn new() -> Self {
        Self {
            input: InputHandler::new()
        }
    }
    
    pub fn with_title(self, _title: &str) -> Self {
        self
    }
    
    pub fn run<A: App>(mut self, mut app: A) -> Result<(), Box<dyn std::error::Error>> {
        let mut ui = Ui::new();
        let mut last_frame = String::new();
        
        loop {
            let wait = std::time::Duration::from_millis((1000 / app.get_fps()).into());
            let input = self.input.poll(wait);
            let current_frame = ui.render();

            if !app.update(input) {
                break;
            }
            
            ui.clear();
            app.display(&mut ui);
            if current_frame != last_frame {
                clear_screen();
                print!("{}", current_frame);
                io::stdout().flush().unwrap();
                last_frame = current_frame;
            }
            
            std::thread::sleep(wait);
        }
        
        Ok(())
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[3J\x1b[1;1H"); 
}