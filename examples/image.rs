#![feature(globs)]

extern crate graphics;
extern crate piston;

use graphics::*;
use piston::{
    AssetStore,
    Game,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
    Gl,
    Texture,
};

pub struct App {
    image: Option<Texture>,
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App {
            image: None,
        }
    }
}

impl Game for App {
    fn load(&mut self, asset_store: &mut AssetStore) {
        let image = asset_store.path("rust-logo.png").unwrap();
        self.image = Some(Texture::from_path(&image).unwrap());
    }

    fn render(&self, _ext_dt: f64, c: &Context, gl: &mut Gl) {
        c.image(self.image.as_ref().unwrap()).draw(gl);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");

    let mut app = App::new();
    app.run(&mut window, &mut asset_store);
}

