mod automaton;
mod buildings;
mod camera;
mod coords;
mod terrain;
mod units;
mod window;

use camera::Camera;
use coords::Coords;
use terrain::Terrain;
use units::{display_action_queue, ActionQueue, ActionType, JobType, RaceType, Unit};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseState,
    pixels::Color,
    rect::Rect,
};
use std::time::Instant;
use window::{init_sdl2_window, Renderer};

fn main() -> Result<(), String> {
    let mut current_race = RaceType::ANT;
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

    /////////////////////// TERRAIN ///////////////////////////////////////////
    let mut last_time = Instant::now();

    let mut terrain = Terrain::new();
    terrain.generate();
    /////////////////////////////////////////////////////////

    /////////////////////// BUILDINGS //////////////////////////////////////////
    //
    //
    //
    /////////////////////////////////////////////////////////

    /////////////////////// UNITS /////////////////////////////////////////////
    let mut units_list: Vec<Unit> = Vec::new();

    for _ in 0..100 {
        let mut unit = Unit::new();
       //      unit.race = RaceType::HUMAN;
       // unit.job = JobType::MINER(terrain::MineralType::IRON);
        unit.action_queue.do_now(ActionType::WANDER, unit.coords);
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
                        for u in &mut units_list {
                            if u.race == current_race {
                                u.action_queue.clear();
                                u.action_queue.do_now(
                                    ActionType::MOVE,
                                    Coords {
                                        x: (x as f32 * camera.zoom) as i32,
                                        y: (y as f32 * camera.zoom) as i32,
                                    },
                                );
                            }
                        }

                        dragging = false;
                        renderer.all_need_update();
                    } else if mouse_btn == sdl2::mouse::MouseButton::Right {
                        for u in &mut units_list {
                            if u.race == current_race
                            //            && u.job == JobType::MINER(terrain::MineralType::IRON)
                            {
                                //  u.action_queue.clear();
                                u.action_queue.do_now(
                                    ActionType::DIG,
                                    Coords {
                                        x: (x as f32 * camera.zoom) as i32,
                                        y: (y as f32 * camera.zoom) as i32,
                                    },
                                );
                            }
                        }
                    }
                    /*
                    println!(
                        "camera pos : ({:?},{:?}) / zoom ({:?})",
                        camera.position.x, camera.position.y, camera.zoom
                    );
                    */
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
                    renderer.draw(&terrain, units_list.clone(), &camera);
                    continue;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    for u in units_list.iter_mut() {
                        if u.race == current_race {
                            renderer.render_text("Dig", 0, 0)?;
                            u.action_queue.insert(0, u.job.get_action(&terrain, &u));
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => current_race = RaceType::ANT,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => current_race = RaceType::HUMAN,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => current_race = RaceType::ALIEN,

                _ => {}
            }
        }

        renderer.canvas.set_draw_color(match current_race {
            RaceType::HUMAN => Color::BLUE,
            RaceType::ANT => Color::RED,
            RaceType::ALIEN => Color::GREEN,
        });

        renderer.draw(&terrain, units_list.clone(), &camera);
        renderer.canvas.fill_rect(Rect::new(0, 0, 50, 50))?;

        for u in units_list.iter_mut() {
            if u.last_action_timer == 0 && u.action_queue.len() > 0 {
                //  BROKEN ... :(  u.action_queue.keep_only(vec![ActionType::MOVE, ActionType::WANDER, ActionType::DIG]);
                //  BROKEN ... :( u.action_queue.remove_only(vec![ActionType::WANDER]);
                display_action_queue(current_race, u.clone());
            }
            u.think(&mut terrain, delta_time);
        }
        renderer.units.needs_update = true;

        renderer.canvas.present();
        // A draw a rectangle which almost fills our window with it !
    }
}
