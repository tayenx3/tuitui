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
        if let None = key {
            return true
        }
        let key = key.unwrap();

        let mut quit = false;

        match key.key {
            InputKey::Esc => quit = true,
            _ => {}
        }
        !quit
    }

    fn get_fps(&self) -> u32 {
        60
    }
}

fn main() -> Result<()> {
    AppRuntime::new().run(Mascot)
}