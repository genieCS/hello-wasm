use wasm_bindgen::prelude::*;
use cursive:: {
    backends,
    backend::Backend,
    Cursive,
    Printer,
    Vec2,
    View,
    theme::{BaseColor, Color, ColorStyle},
};
use std::sync::Mutex;
use web_sys::console;

#[wasm_bindgen]
pub struct RectangleWrapper {
    backend: Mutex<Cursive>,
}

#[wasm_bindgen]
impl RectangleWrapper {
    #[wasm_bindgen(js_name = "rectangle")]
    pub async fn rectangle() -> RectangleWrapper {
        console::log_1(&"Hello from Rust!".into());
        let mut siv: Cursive = Cursive::new();
        siv.add_layer(Rectangle {});
        let siv: Mutex<Cursive> = std::sync::Mutex::new(siv);
        siv.lock().unwrap().run_with(|| backend()).await;
        RectangleWrapper { backend: siv }
    }
}

struct Rectangle {}

impl View for Rectangle {
    fn draw(&self, printer: &Printer) {
        printer.with_color(ColorStyle::new(Color::Dark(BaseColor::Green), Color::Dark(BaseColor::Green)), |printer| {
            printer.print((0,0), "            ");
        });
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        Vec2::new(12,12)
    }
}

pub fn backend() -> Box<dyn Backend> {
    backends::wasm::Backend::init().unwrap()
}