use tuitui::prelude::*;

#[app(
    name = "Button Game",
    fps = 60,
    quit_keys = ["q"]
)]
struct MyApp {
    text: String
}

impl App for MyApp {
    fn display(&mut self, ui: &mut Ui) {
        ui
            .heading("Tuitui Button Game 🐧")
            .separator("=")
            .heading("Press `Q` to quit")
            .horizontal_container(|ui| {
                ui
                    .button("Click me!")
                        .on_click(|| println!("Clicked!"))
                    .text_input()
                        .placeholder("Type...")
                        .bind(&mut self.text);
            })
            .text(&format!("Hello, {}!", self.text));
    }
}

fn main() {
    MyApp::new().run();
}