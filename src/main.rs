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

fn main() -> Result<(), String> {
    let mut camera = Camera::new(window::WIDTH, window::HEIGHT);

    let mut camera_x: i32 = 0;
    let mut camera_y: i32 = 0;
    let mut dragging = false;
    let mut prev_mouse_x = 0;
    let mut prev_mouse_y = 0;

    let mut unit_list: Vec<Unit> = Vec::new();

    let (sdl_context, window) = window::init_sdl2_window();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut last_time = Instant::now();

    let mut terrain = Terrain::new();
    terrain.generate();

    for i in 0..10 {
        let mut coords = Coords {
            x: (window::WIDTH / 2) as i32,
            y: (window::HEIGHT / 2) as i32,
        };

        for _ in 0..10 {
            let x = rand::thread_rng().gen_range(0..window::WIDTH - 1);
            let y = rand::thread_rng().gen_range(0..window::HEIGHT - 1);
            coords = Coords {
                x: x as i32,
                y: y as i32,
            };

            if terrain.get_data(x as usize, y as usize) == Some(TileType::AIR) {
                break;
            }
        }

        let mut unit = Unit::new(
            if i % 3 == 0 {
                RaceType::HUMAN
            } else if i % 3 == 1 {
                RaceType::ANT
            } else {
                RaceType::ALIEN
            },
            JobType::MINER,
            coords,
        );

        for _ in 0..10 {
            unit.action_queue.push(ActionType::WANDER);
        }
        unit_list.push(unit);
    }

    let texture_creator = canvas.texture_creator();
    let mut terrain_texture = texture_creator
        .create_texture_target(None, window::WIDTH, window::HEIGHT)
        .expect("Failed to create texture");
    canvas
        .with_texture_canvas(&mut terrain_texture, |texture_canvas| {
            terrain.draw(texture_canvas, &texture_creator, &camera);
        })
        .expect("Failed to draw terrain on texture");

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

                        camera_x += (delta_x as f32) as i32;
                        camera_y += (delta_y as f32) as i32;

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
                    let zoom = if y > 0 { 1.1 } else { 0.9 };
                    camera.apply_zoom(zoom, mouse_x, mouse_y);
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
                    terrain.generate();
                    canvas
                        .with_texture_canvas(&mut terrain_texture, |texture_canvas| {
                            terrain.draw(texture_canvas, &texture_creator, &camera);
                        })
                        .expect("Failed to draw terrain on texture");
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
                    canvas
                        .with_texture_canvas(&mut terrain_texture, |texture_canvas| {
                            terrain.draw(texture_canvas, &texture_creator, &camera);
                        })
                        .expect("Failed to draw terrain on texture");
                    println!(
                        "threshold : {:?} ||| scale : {:?} ||| iteration",
                        terrain.minerals[2].automaton.perlin_threshold,
                        terrain.minerals[2].automaton.perlin_scale
                    );

                }

                _ => {}
            }
        }

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_millis() as i32;
        last_time = current_time;

        for u in &mut unit_list {
            u.think(terrain.clone(), delta_time);
            u.draw_at(&mut canvas, camera.zoom).expect("cant draw unit");
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_viewport(Some(Rect::new(
            camera_x,
            camera_y,
            (window::WIDTH as f32 * camera.zoom) as u32,
            (window::HEIGHT as f32 * camera.zoom) as u32,
        )));
        let terrain_dst = Rect::new(
            0,
            0,
            (window::WIDTH as f32 * camera.zoom) as u32,
            (window::HEIGHT as f32 * camera.zoom) as u32,
        );

        canvas
            .copy(&terrain_texture, None, terrain_dst)
            .expect("Failed to copy texture");

        canvas.present();
    }
}
