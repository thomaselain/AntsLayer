mod automaton;
mod camera;
mod coords;
mod terrain;
mod units;
mod window;
mod buildings;

use buildings::FindHome;
use camera::Camera;
use coords::Coords;
use terrain::{Terrain, TileType};
use units::{ActionType, Actions, RaceType, Unit};

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

    /////////////////////// BUILDINGS ///////////////////////////////////////////
    let mut last_time = Instant::now();

    let mut terrain = Terrain::new();
    terrain.generate();
    /////////////////////////////////////////////////////////



    /////////////////////// BUILDINGS //////////////////////////////////////////
    for b in terrain.buildings.clone() {
        terrain.data[b.1.coords.x as usize][b.1.coords.y as usize] = TileType::Building(b.1.building_type);
        terrain.dig_home(b.1.coords, 15);
    }
    /////////////////////////////////////////////////////////



    /////////////////////// UNITS /////////////////////////////////////////////
    let mut units_list: Vec<Unit> = Vec::new();

    for _ in 0..100 {
        let mut unit = Unit::new();
        /*

        for _ in 0..1 {
            unit.action_queue
                .push((ActionType::WANDER, Coords { x: 0, y: 0 }));
        }
        */
      
        unit.action_queue.push((
               ActionType::MOVE,
               Coords {
                   x: terrain::WIDTH as i32 / 2,
                   y: terrain::HEIGHT as i32 / 2,
               },
           ));
        //unit.race = RaceType::ANT;
        units_list.push(unit);
    }
    /////////////////////////////////////////////////////////
    renderer.all_need_update();

    '_main: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_millis() as i32;
        last_time = current_time;
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
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        dragging = false;
                        renderer.all_need_update();
                    } else if mouse_btn == sdl2::mouse::MouseButton::Right {
                        for u in &mut units_list {
                            //if u.race == RaceType::ANT {
                                u.action_queue.push((
                                    ActionType::MOVE,
                                    Coords {
                                        x: (x as f32 * camera.zoom) as i32,
                                        y: (y as f32 * camera.zoom) as i32,
                                    },
                                ));
                            //}
                        }
                    }
                    println!(
                        "camera pos : ({:?},{:?}) / zoom ({:?})",
                        camera.position.x, camera.position.y, camera.zoom
                    );
                }
                Event::MouseMotion { .. } => {
                    if dragging {
                        let delta_y = mouse_y - prev_mouse_y;
                        let delta_x = mouse_x - prev_mouse_x;

                        camera.position.x = camera.position.x - delta_x as i32;
                        camera.position.y = camera.position.y - delta_y as i32;

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

        renderer.units.needs_update = true;
        units_list.think(&terrain, delta_time);

        renderer.draw(&terrain, &units_list, &camera);
    }
}
