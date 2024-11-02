mod automaton;
mod camera;
mod coords;
mod terrain;
mod units;
mod window;

use camera::Camera;
use coords::Coords;
use terrain::{Mineral, Terrain, TileType};
use units::{ActionType, Actions, JobType, RaceType, Unit};

use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseState, pixels::Color, rect::Rect};
use std::time::Instant;
use window::{init_sdl2_window, Renderer};

fn main() -> Result<(), String> {
    let mut dragging = false;
    let mut prev_mouse_x = 0;
    let mut prev_mouse_y = 0;

    let mut camera = Camera::new(window::WIDTH, window::HEIGHT);
    let (sdl_context, win) = init_sdl2_window();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut renderer: Renderer = Renderer::new(
        (sdl_context, win),
        window::WIDTH as usize,
        window::HEIGHT as usize,
    );

    let mut last_time = Instant::now();

    let mut terrain = Terrain::new();

    'main: loop {
        let mouse_state = MouseState::new(&event_pump);

        for event in event_pump.poll_iter() {
            let mouse_x = mouse_state.x();
            let mouse_y = mouse_state.y();

            match event {
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        dragging = true;
                        prev_mouse_x = x;
                        prev_mouse_y = y;
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        dragging = false;
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    if dragging {
                        let delta_x = x - prev_mouse_x;
                        let delta_y = y - prev_mouse_y;

                        camera.position.x += (delta_x as f32) as i32;
                        camera.position.y += (delta_y as f32) as i32;

                        prev_mouse_x = x;
                        prev_mouse_y = y;
                    }
                }
                Event::MouseWheel { y, .. } => {
                    if y > 0 && camera.zoom < 10.0 {
                        camera.zoom += 0.1;
                    } else if y < 0 && camera.zoom > 0.5 {
                        camera.zoom -= 0.1;
                    }
                    camera.zoom = if y > 0 { 1.1 } else { 0.9 };
                    camera.apply_zoom(mouse_x, mouse_y);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Ok(());
                }
                //////////////// Regenerate terrain after its modification (for testing)
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::A),
                    ..
                }
                | sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Z),
                    ..
                } => {
                    renderer.draw(&camera);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    terrain.minerals[2].automaton.perlin_scale += 0.01;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    terrain.minerals[2].automaton.perlin_scale -= 0.005;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    camera.zoom = 1.0;
                    terrain = Terrain::new();
                    terrain.generate();

                    renderer.terrain.draw_terrain(&terrain);
                    renderer.draw(&camera);
                }

                _ => {}
            }
        }

        let current_time = Instant::now();
        let last_time = current_time;
        let delta_time = current_time.duration_since(last_time).as_millis() as i32;
        renderer.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        renderer.canvas.clear();
        println!("{:?}", renderer.terrain.needs_update);

        renderer.draw(&camera);
        renderer.canvas.set_viewport(Some(Rect::new(
            camera.position.x,
            camera.position.y,
            (window::WIDTH as f32 * camera.zoom) as u32,
            (window::HEIGHT as f32 * camera.zoom) as u32,
        )));

        renderer.canvas.present();
    }
}
