use coords::Coords;
use main_menu::create_main_menu_buttons;
use sdl2::{ pixels::Color, rect::{ Point, Rect } };

use super::render::window::Renderer;

pub(crate) mod main_menu;

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub enum ButtonId {
    Exit,
    Start,
    Settings,
}
#[derive(Clone)]
pub struct Button {
    rect: Rect,
    text: String,
    color: Color,
    id: ButtonId,
}

impl Button {
    pub fn new(id: ButtonId) -> Button {
        match id {
            ButtonId::Start => {
                Button {
                    rect: Rect::new(300, 0, 300, 100),
                    text: "Start".to_string(),
                    color: Color::GREEN,
                    id: ButtonId::Start,
                }
            }
            ButtonId::Settings => {
                Button {
                    rect: Rect::new(300, 200, 300, 100),
                    text: "Settings".to_string(),
                    color: Color::GRAY,
                    id: ButtonId::Settings,
                }
            }
            ButtonId::Exit => {
                Button {
                    rect: Rect::new(300, 600, 300, 100),
                    text: "Close".to_string(),
                    color: Color::RED,
                    id: ButtonId::Exit,
                }
            }
        }
    }

    pub fn clicked(self, coords: Coords) -> Result<ButtonId, ()> {
        match self.rect.contains_point(Point::new(coords.x(), coords.y())) {
            true => { Ok(self.id) }
            false => Err(()),
        }
    }
    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.canvas.set_draw_color(self.clone().color);
        let _ = renderer.canvas.fill_rect(self.clone().rect);
        renderer.canvas.set_draw_color(Color::BLACK);
        let _ = renderer.render_text(
            &self.clone().text,
            self.clone().rect.x + self.clone().rect.w / 2,
            self.clone().rect.y + self.clone().rect.h / 2
        );
    }
}
pub struct Menu {
    pub buttons: Vec<Button>,
}

impl Menu {}
