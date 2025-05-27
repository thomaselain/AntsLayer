use sdl2::{ pixels::Color, rect::Rect };

use crate::renderer::Renderer;

const BACKGROUND_COLOR: Color = Color::BLACK;

#[allow(unused)]
pub struct Interface {
    buttons: Vec<Button>,
}
#[allow(unused)]
#[derive(Clone, Copy)]
struct Button {
    t: ButtonType,
    pos: (i32, i32),
    c: Color,
}
impl Button {
    pub fn draw(&self, renderer: &mut Renderer) {
        let (x, y) = self.pos;
        let (w,h) = renderer.dims;

        renderer.canvas.set_draw_color(self.c);
        renderer.canvas.draw_rect(Rect::new(x, y, w, h)).unwrap();
        renderer.canvas.set_draw_color(Color::BLACK);
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
enum ButtonType {
    SLIDER,
    TOGGLE,
    SINGLE,
}

#[allow(unused)]
impl Interface {
    pub fn new() -> Self {
        Self { buttons: vec![] }
    }
    pub fn draw(&self, renderer: &mut Renderer) {
        for b in self.buttons.clone() {
            b.draw(renderer);
        }
    }
}
