use sdl2::{ pixels::Color, rect::Rect, render:: Canvas  };

#[derive(Clone)]
pub(crate) struct Button{
    pub label: String,
    pub rect: Rect,
    pub color: Color,
}

impl Button {
    pub fn new(label: &str, x: i32, y: i32, w: i32, h: i32, color: Color) -> Self {
        let rect = Rect::new(x, y, w as u32, h as u32);
        Button {
            label: label.to_string(),
            rect,
            color,
        }
    }


    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        // Dessiner le bouton
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect).unwrap();
    }

    pub fn is_clicked(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point(sdl2::rect::Point::new(x, y))
    }
}
