mod game;

// #![allow(unused_imports)]

use coords::Coords;
use game::{
    interface::main_menu::main_menu,
    map::Map,
    render::{ camera::Camera, window::{ self, init_sdl2_window, Renderer } },
    units::{ actions::{ Action, ActionQueue, ActionType }, RaceType, Unit },
};
//use team::Team;
use sdl2::{ event::Event, keyboard::Keycode, mouse::MouseState, pixels::Color, rect::Rect };
use std::time::Instant;

fn main() -> Result<(), String> {
    // LOOP in main menu
    main_menu().expect("...?");
    //

    let mut current_race = RaceType::ANT;
    let mut dragging = false;
    let mut prev_mouse_x = 0;
    let mut prev_mouse_y = 0;

    let mut camera = Camera::new(window::WIDTH as u32, window::HEIGHT as u32);
    let (sdl_context, win) = init_sdl2_window();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut renderer: Renderer = Renderer::new(
        (sdl_context, win),
        window::WIDTH as usize,
        window::HEIGHT as usize
    );

    let mut last_time = Instant::now();

    let mut map = Map::new();
    map.generate().expect("Map generation failed for some reason");

    let mut units_list: Vec<Unit> = Vec::new();

    for _ in 0..1 {
        let race = None;
        //let race = RaceType::ANT;
        let mut unit = Unit::new(race);
        //unit.job = JobType::MINER(TileType::Mineral(MineralType::ROCK));
        unit.coords = unit.race.starting_coords();
        unit.action_queue.do_now(Action(ActionType::MOVE, unit.race.starting_coords()));
        units_list.push(unit);
    }

    renderer.all_need_update();

    '_main: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_millis() as i32;
        last_time = current_time;
        renderer.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        //  renderer.canvas.clear();

        let mouse_state = MouseState::new(&event_pump);

        for event in event_pump.poll_iter() {
            let mouse_x = mouse_state.x();
            let mouse_y = mouse_state.y();

            match event {
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        dragging = true;
                        prev_mouse_x = x;
                        prev_mouse_y = y;
                        renderer.all_need_update();
                    }
                }
                Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        if !dragging {
                            for u in &mut units_list {
                                if u.race == current_race {
                                    u.action_queue.clear();
                                    u.action_queue.do_now(
                                        Action(
                                            ActionType::MOVE,
                                            Coords(
                                                ((x as f32) * camera.zoom) as i32,
                                                ((y as f32) * camera.zoom) as i32
                                            )
                                        )
                                    );
                                }
                            }
                        }

                        dragging = false;
                        renderer.all_need_update();
                    } else if mouse_btn == sdl2::mouse::MouseButton::Right {
                        for u in &mut units_list {
                            if
                                u.race == current_race
                                //            && u.job == JobType::MINER(terrain::MineralType::IRON)
                            {
                                // u.action_queue.clear();
                                u.action_queue.do_now(
                                    Action(
                                        ActionType::DIG,
                                        Coords(
                                            ((x as f32) * camera.zoom) as i32,
                                            ((y as f32) * camera.zoom) as i32
                                        )
                                    )
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

                        camera.position.0 = camera.position.x() - (delta_x as i32);
                        camera.position.1 = camera.position.y() - (delta_y as i32);

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
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(());
                }
                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    let joe = Unit::new(Some(current_race));
                    units_list.push(joe);
                }
                Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                    map = Map::new();
                    map.generate().expect("Map generation failed for some reason");
                    renderer.all_need_update();
                    renderer.draw(map.clone(), units_list.clone(), &camera);
                    continue;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    for u in units_list.iter_mut() {
                        if u.race == current_race && u.job.get_miner_target().is_ok() {
                            // renderer.render_text("Dig", 0, 0)?;
                            let tile_type = u.job.get_miner_target().expect("...").to_tile_type();

                            let x = map.find_closest(u.coords, tile_type);
                            let Some(coords) = x.ok() else {
                                continue;
                            };

                            println!("DIG");
                            u.action_queue.clear();
                            if let Ok(job) = u.find_job_action(&map) {
                                u.action_queue.do_now(Action(job.0, job.1));
                            } // u.action_queue.do_now(Action(ActionType::MOVE, coords));
                            //    u.action_queue.do_now(Action(ActionType::DIG, coords));
                            // u.action_queue.do_later(Action(ActionType::HAUL, u.coords));
                        }
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    current_race = RaceType::ANT;
                }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    current_race = RaceType::HUMAN;
                }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    current_race = RaceType::ALIEN;
                }

                _ => {}
            }
        }

        renderer.canvas.set_draw_color(match current_race {
            RaceType::HUMAN => Color::BLUE,
            RaceType::ANT => Color::RED,
            RaceType::ALIEN => Color::GREEN,
        });

        renderer.draw(map.clone(), units_list.clone(), &camera);
        renderer.canvas.fill_rect(Rect::new(0, 0, 50, 50))?;

        for u in units_list.iter_mut() {
            if u.last_action_timer == 0 && u.action_queue.len() > 0 {
                //  BROKEN ... :(  u.action_queue.keep_only(vec![ActionType::MOVE, ActionType::WANDER, ActionType::DIG]);
                //  BROKEN ... :( u.action_queue.remove_only(vec![ActionType::WANDER]);
                //
                //
                // display_action_queue(current_race, u.clone());
            }
            u.think(&mut map, delta_time).ok();
            println!("{:?}", u.inventory.0.len());
        }
        //renderer.all_need_update();

        renderer.canvas.present();
        // A draw a rectangle which almost fills our window with it !
    }
}
