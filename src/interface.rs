use sdl2::{ pixels::Color, rect::{ Point, Rect } };

use crate::{ renderer::Renderer };

// #[allow(unused)]
const BACKGROUND_COLOR: Color = Color::BLACK;

#[allow(unused)]
pub struct Interface {
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            buttons: vec![Button::zoom_in(), Button::zoom_out()],
            sliders: vec![Slider {
                x: 0,
                y: 0,
                width: 150,
                min: 0.0,
                max: 1.0,
                value: 0.5,
                is_dragging: false,
            }],
        }
    }
}

#[derive(Clone, Copy)]
pub struct Slider {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub is_dragging: bool,
}
impl Slider {
    pub fn contains_point(&self, px: i32, py: i32) -> bool {
        let normalized = (self.value - self.min) / (self.max - self.min);
        let handle_x = self.x + ((normalized * (self.width as f32)) as i32);
        let handle_rect = Rect::new(handle_x - 5, self.y - 4, 10, 14);
        handle_rect.contains_point(Point::new(px, py))
    }
}

impl Interface {
    pub fn check_sliders_at(&mut self, (x, y): (i32, i32)) -> Option<usize> {
        for (i, s) in self.sliders.iter_mut().enumerate() {
            if s.contains_point(x, y) {
                s.is_dragging = true;
                return Some(i);
            }
        }
        None
    }
        pub fn clear_sliders_state(&mut self) {
        for s in &mut self.sliders {
            s.is_dragging = false;
        }
    }
}

impl Slider {
    pub fn draw(&mut self, renderer: &'_ mut Renderer) {
        let rect = Rect::new(self.x, self.y, self.width as u32, 6);
        renderer.canvas.set_draw_color(Color::GRAY);
        let _ = renderer.canvas.fill_rect(rect);

        let normalized = (self.value - self.min) / (self.max - self.min);
        let handle_x = self.x + ((normalized * (self.width as f32)) as i32);

        let handle_rect = Rect::new(handle_x - 5, self.y - 4, 10, 14);
        renderer.canvas.set_draw_color(Color::WHITE);
        let _ = renderer.canvas.fill_rect(handle_rect);
    }
}
impl Interface {
    pub fn render(&mut self, renderer: &mut Renderer) {
        for b in &self.buttons {
            b.draw(renderer);
        }
        for s in &mut self.sliders {
            s.draw(renderer);
        }
    }
}
pub struct Button {
    rect: Rect,
    color: Color,
    label: String,
    action: ButtonAction,
}

#[allow(unused)]
#[derive(Clone, Copy)]
enum ButtonType {
    SLIDER,
    TOGGLE,
    SINGLE,
}

type ButtonAction = Box<dyn FnMut(&mut Renderer) -> Result<(), ()>>;
impl Button {
    pub fn zoom_in() -> Self {
        Self {
            rect: (0, 0, 30, 30).into(),
            color: Color::RGBA(255, 25, 120, 200),
            label: "+".to_string(),
            action: Box::new(|renderer: &mut Renderer| { renderer.zoom_in() }),
        }
    }
    pub fn zoom_out() -> Self {
        Self {
            rect: (0, 51, 30, 30).into(),
            color: Color::RGBA(255, 25, 120, 200),
            label: "-".to_string(),
            action: Box::new(|renderer: &mut Renderer| { renderer.zoom_out() }),
        }
    }

    pub fn new(
        (x, y, w, h): (i32, i32, u32, u32),
        label: &str,
        color: Color,
        action: ButtonAction
    ) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            color,
            label: label.to_string(),
            action,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.canvas.set_draw_color(self.color);
        renderer.canvas.fill_rect(self.rect).expect("Failed to draw rect");

        // Centrage grossier du texte dans le bouton
        let label_x = self.rect.x + (self.rect.width() as i32) / 4;
        let label_y = self.rect.y + (self.rect.height() as i32) / 4;

        renderer.draw_text(&self.label, label_x, label_y);

        renderer.canvas.set_draw_color(Color::BLACK);
    }

    pub fn handle_click(&mut self, renderer: &mut Renderer, x: i32, y: i32) -> Result<(), ()> {
        if self.rect.contains_point((x, y)) {
            (self.action)(renderer)?;
        }
        Ok(())
    }
}
