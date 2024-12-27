use std::str::FromStr;

use sdl2::{ pixels::Color, rect::{ Point, Rect }, render::Canvas };

#[derive(Clone)]
pub struct Output(pub Option<OutputType>);

#[derive(Clone, Debug)]
pub enum MenuError {
    FailedToOpen,
    InvalidOutput,
    // Etc ...
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MenuLabel {
    MainMenu,
    MapEditor,
    LoadWorld,
    Settings,
    Exit,
}

impl Into<Button> for MenuLabel {
    fn into(self) -> Button {
        let output = self.clone().into();
        let label = self.clone().into();
        let center = self.clone().into();
        let rect = self.clone().into();
        let color = self.clone().into();

        Button {
            output,
            label,
            center,
            rect,
            color,
            // do_button_stuff
        }
    }
}

// Center  coords
impl Into<Point> for MenuLabel {
    fn into(self) -> Point {
        match self {
            MenuLabel::MapEditor => Point::new(300, 50),
            MenuLabel::MainMenu => Point::new(300, 150),
            MenuLabel::LoadWorld => Point::new(300, 250),
            MenuLabel::Settings => Point::new(300, 350),
            MenuLabel::Exit => Point::new(300, 450),
        }
    }
}

// Color
impl Into<Color> for MenuLabel {
    fn into(self) -> Color {
        match self {
            MenuLabel::MainMenu => Color::RGB(150, 150, 150),
            MenuLabel::MapEditor => Color::RGB(50, 215, 50),
            MenuLabel::LoadWorld => Color::RGB(10, 55, 250),
            MenuLabel::Settings => Color::RGB(75, 75, 75),
            MenuLabel::Exit => Color::RGB(250, 20, 30),
        }
    }
}

// Title
impl Into<String> for MenuLabel {
    fn into(self) -> String {
        match self {
            MenuLabel::MainMenu => { String::from_str("Main menu").unwrap() }
            MenuLabel::MapEditor => { String::from_str("Map editor").unwrap() }
            MenuLabel::LoadWorld => { String::from_str("Load world").unwrap() }
            MenuLabel::Settings => { String::from_str("Settings").unwrap() }
            MenuLabel::Exit => { String::from_str("Exit").unwrap() }
        }
    }
}

// Returned value
impl Into<Output> for MenuLabel {
    fn into(self) -> Output {
        match self {
            MenuLabel::MainMenu => { Output(None) }
            MenuLabel::MapEditor => { Output(None) }
            MenuLabel::LoadWorld => { Output(None) }
            MenuLabel::Settings => { Output(None) }
            MenuLabel::Exit => { Output(None) }
        }
    }
}

impl Into<Rect> for MenuLabel {
    fn into(self) -> Rect {
        Rect::from_center::<Point>(self.into(), 200, 50)
    }
}

#[derive(Clone)]
pub enum OutputType {
    Float,
    Integer,
    String,
    Color,
}

#[derive(Clone)]
pub(crate) struct Button {
    pub output: Output,
    pub label: MenuLabel,
    pub center: Point,
    pub rect: Rect,
    pub color: Color,
    // pub do_button_stuff: fn() -> Output,
    // Hmmmmm
}

impl Button {
    pub fn new(label: MenuLabel) -> Self {
        label.into()
    }
}

impl Button {
    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        // Dessiner le bouton
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect).unwrap();
    }

    pub fn is_clicked(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point(sdl2::rect::Point::new(x, y))
    }
}
