use std::collections::HashMap;

use sdl2::{ pixels::Color, rect::{ Point, Rect } };

use crate::{ chunk::{ HEIGHT, SEA_LEVEL }, renderer::{ Renderer, DEFAULT_TILE_SIZE } };

///////////////////////////////////////////////////////
type InterfaceAction = Box<dyn FnMut(&mut Renderer) -> Result<(), ()>>;
///////////////////////////////////////////////////////

#[allow(unused)]
const BACKGROUND_COLOR: Color = Color::BLACK;

const CURSOR_HEIGHT: i32 = 20;
const CURSOR_WIDTH: i32 = CURSOR_HEIGHT / 2;
const SLIDER_WIDTH: i32 = 250;

#[derive(Copy, Clone, Hash, Ord, PartialEq, PartialOrd, Eq)]
pub enum Id {
    Zoom,
    CameraZ,
    Plus,
    Minus,
}

#[allow(unused)]
pub struct Interface {
    pub buttons: HashMap<Id, Button>,
    pub sliders: HashMap<Id, Slider>,
}

impl Interface {
    pub fn render(&mut self, renderer: &mut Renderer) {
        for (_, b) in &self.buttons {
            b.draw(renderer);
        }
        for (_, s) in &mut self.sliders {
            s.draw(renderer);
        }
    }
}

impl Interface {
    pub fn new() -> Self {
        let mut sliders = HashMap::new();

        sliders.insert(Id::Zoom, Slider {
            x: 20,
            y: 300,
            width: SLIDER_WIDTH,
            min: 0,
            max: 100,
            value: DEFAULT_TILE_SIZE as i32,
            is_dragging: false,
            on_change: Some(
                Box::new(|_new_value| {
                    // println!("Zoom level changed to {new_value}");
                })
            ),
        });
        sliders.insert(Id::CameraZ, Slider {
            x: 20,
            y: 350,
            width: SLIDER_WIDTH,
            min: 0,
            max: HEIGHT as i32,
            value: SEA_LEVEL as i32,
            is_dragging: false,
            on_change: Some(Box::new(|_v| {
                // println!("Height changed to  {v}");
            })),
        
        });

        let mut buttons = HashMap::new();
        buttons.insert(Id::Plus, Button::plus());
        buttons.insert(Id::Minus, Button::minus());

        Self {
            buttons,
            sliders,
        }
    }
}

impl Interface {
    pub fn update(&mut self, id: Id, mouse_pos: i32) -> i32 {
        if let Some(slider) = self.sliders.get_mut(&id) {
            match id {
                Id::Zoom | Id::CameraZ => {
                    slider.update(mouse_pos);
                }
                Id::Plus | Id::Minus => todo!("Buttons update"),
            }

            // Return the slider value
            //(Interface usage clarity)
            return slider.value;
        } else {
            panic!("Unknown interface clicked : SHOULD NOT HAPPEN")
        }
    }
}

// #[derive(Hash, Ord, PartialEq, PartialOrd, Eq)]
pub struct Slider {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub min: i32,
    pub max: i32,
    pub value: i32,
    pub is_dragging: bool,
    pub on_change: Option<Box<dyn FnMut(i32)>>,
}

impl Slider {
    pub fn draw(&mut self, renderer: &'_ mut Renderer) {
        let rect = Rect::new(self.x, self.y, self.width as u32, 10);
        renderer.canvas.set_draw_color(Color::GRAY);
        let _ = renderer.canvas.fill_rect(rect);

        let normalized = ((self.value - self.min) as f32) / ((self.max - self.min) as f32);
        let handle_x = self.x + ((normalized * (self.width as f32)) as i32);

        let handle_rect = Rect::new(
            handle_x - CURSOR_WIDTH / 2,
            self.y - CURSOR_HEIGHT / 4,
            CURSOR_WIDTH as u32,
            CURSOR_HEIGHT as u32
        );

        let value_label = format!("{}", self.value);
        renderer.draw_text(&value_label, self.x - 15, self.y - 5);

        renderer.canvas.set_draw_color(Color::WHITE);
        let _ = renderer.canvas.fill_rect(handle_rect);
    }
}

impl Slider {
    pub fn contains_point(&self, px: i32, py: i32) -> bool {
        let normalized = ((self.value - self.min) as f32) / ((self.max - self.min) as f32);
        let handle_x = self.x + ((normalized * (self.width as f32)) as i32);
        let handle_rect = Rect::new(handle_x - 5, self.y - 4, 10, 14);
        handle_rect.contains_point(Point::new(px, py))
    }

    
    fn update(&mut self, mouse_pos: i32) {
        let clamped_x = (mouse_pos - self.x).clamp(0, self.width);
        let ratio = (clamped_x as f32) / (self.width as f32);
        let new_value = self.min + ((((self.max - self.min) as f32) * ratio).round() as i32);

        if new_value != self.value {
            self.value = new_value;
            if let Some(callback) = &mut self.on_change {
                callback(self.value);
            }
        }
    }

    // fn update(&mut self, mouse_pos: i32) {
    //     let clamped_x = (mouse_pos - self.x).clamp(0, self.width);
    //     let ratio = (clamped_x as f32) / (self.width as f32);

    //     self.value = self.min + ((((self.max - self.min) as f32) * ratio) as i32);
    // }
}

impl Interface {
    pub fn check_sliders_at(&mut self, (x, y): (i32, i32)) -> Option<Id> {
        for (_i, (id, s)) in self.sliders.iter_mut().enumerate() {
            if s.contains_point(x, y) {
                s.is_dragging = true;
                return Some(*id);
            }
        }
        None
    }
    pub fn clear_sliders_state(&mut self) {
        for (_, s) in &mut self.sliders {
            s.is_dragging = false;
        }
    }
}

pub struct Button {
    rect: Rect,
    color: Color,
    label: String,
    action: InterfaceAction,
}

#[allow(unused)]
#[derive(Clone, Copy)]
enum ButtonType {
    SLIDER,
    TOGGLE,
    SINGLE,
}

impl Button {
    pub fn plus() -> Self {
        Self {
            rect: (0, 175, 30, 30).into(),
            color: Color::RGBA(255, 25, 120, 255),
            label: "+".to_string(),
            action: Box::new(|renderer: &mut Renderer| { renderer.increase_view_dist() }),
        }
    }
    pub fn minus() -> Self {
        Self {
            rect: (0, 200, 30, 30).into(),
            color: Color::RGBA(255, 25, 120, 255),
            label: "-".to_string(),
            action: Box::new(|renderer: &mut Renderer| { renderer.decrease_view_dist() }),
        }
    }

    pub fn new(
        (x, y, w, h): (i32, i32, u32, u32),
        label: &str,
        color: Color,
        action: InterfaceAction
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
