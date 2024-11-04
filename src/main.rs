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

use crate::{terrain::HEIGHT, terrain::WIDTH};
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

    let last_time = Instant::now();

    let mut terrain = Terrain::new();
    terrain.generate();

    /////////////////////// UNITS /////////////////////////////
    let mut units_list: Vec<Unit> = Vec::new();

    for i in 0..100 {
        let mut unit = Unit::new();

        for _ in 0..1 {
            unit.action_queue.push(ActionType::WANDER);
        }
        units_list.push(unit);
    }
    /////////////////////////////////////////////////////////
    renderer.all_need_update();

    'main: loop {
        renderer.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        renderer.canvas.clear();

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
                        renderer.all_need_update();
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        dragging = false;
                        renderer.all_need_update();
                    }
                    println!(
                        "camera pos : ({:?},{:?})",
                        camera.position.x, camera.position.y
                    );
                }
                Event::MouseMotion { x, y, .. } => {
                    if dragging {
                        let delta_y = mouse_y - prev_mouse_y;
                        let delta_x = mouse_x - prev_mouse_x;

                        camera.position.x = (camera.position.x - delta_x as i32);
                        camera.position.y = (camera.position.y - delta_y as i32);

                        prev_mouse_x = mouse_x;
                        prev_mouse_y = mouse_y;
                    }
                }
                Event::MouseWheel { y, .. } => {
                    if y > 0 {
                        camera.zoom_in();
                    } else if y < 0 {
                        camera.zoom_out();
                    }
                    println!("{:?}", camera.zoom);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Ok(());
                }

                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    terrain = Terrain::new();
                    terrain.generate();
                    renderer.all_need_update();
                    renderer.draw(&terrain, &units_list, &camera);
                }

                _ => {}
            }
        }

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_millis() as i32;
        renderer.units.needs_update = true;

        units_list.think(&terrain, delta_time);

        let last_time = current_time;
        renderer.draw(&terrain, &units_list, &camera);
    }
}
