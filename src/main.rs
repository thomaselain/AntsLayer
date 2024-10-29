//mod shaders;
//use shaders::create_shader;

use std::thread;
use std::time::{Duration, Instant};

mod units;
use rand::Rng;
use sdl2::rect::Rect;
use terrain::{Terrain, TileType};
use units::{ActionType, Actions, Coords, JobType, RaceType, Unit};

mod window;

mod terrain;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() -> Result<(), String> {
    let mut camera_x: i32 = 0;
    let mut camera_y: i32 = 0;
    let mut dragging = false;
    let mut prev_mouse_x = 0;
    let mut prev_mouse_y = 0;

    let mut zoom: f32 = 1.0;
    let mut unit_list: Vec<Unit> = Vec::new();

    let (sdl_context, window) = window::init_sdl2_window();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut last_time = Instant::now();

    let texture_creator = canvas.texture_creator();
    let mut terrain_texture = texture_creator
        .create_texture_target(None, window::WIDTH, window::HEIGHT)
        .expect("Failed to create texture");

    let mut terrain = Terrain::new();
    terrain.generate();

    ///////////////////////////////////////////// UNITS CREATION ///////////////////////////////////
    for i in 0..100 {
        let mut coords;

        // Boucle jusqu'à ce qu'on trouve une case de type AIR
        loop {
            let x = rand::thread_rng().gen_range(0..window::WIDTH - 1);
            let y = rand::thread_rng().gen_range(0..window::HEIGHT - 1);
            coords = Coords { x: x as i32, y: y as i32 };

            // Vérifie si la case est de type AIR avant d'assigner
            if terrain.get_data(x as usize, y as usize) == Some(TileType::AIR) {
                break; // Coordonnées valides, on peut sortir de la boucle
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
        // add move actions for testing
        for _ in 0..10 {
            unit.action_queue.push(ActionType::WANDER);
        }
        unit_list.push(unit);
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////

    canvas
        .with_texture_canvas(&mut terrain_texture, |texture_canvas| {
            terrain.draw(texture_canvas);
        })
        .expect("Failed to draw terrain on texture");

    'main: loop {
        for event in event_pump.poll_iter() {
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

                        camera_x += delta_x;
                        camera_y += delta_y;

                        prev_mouse_x = x;
                        prev_mouse_y = y;
                    }
                }
                Event::MouseWheel { y, .. } => {
                    if y > 0 && zoom < 10.0 {
                        zoom += 0.1;
                    } else if y < 0 && zoom > 0.5 {
                        zoom -= 0.1;
                    }
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Ok(());
                }
                _ => {}
            }
        }

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_millis() as i32;
        last_time = current_time;

        // clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_viewport(Some(Rect::new(camera_x, camera_y, window::WIDTH, window::HEIGHT)));
        let terrain_dst = Rect::new(0, 0, (window::WIDTH as f32 * zoom) as u32, (window::HEIGHT as f32 * zoom) as u32);

        canvas
            .copy(&terrain_texture, None, terrain_dst)
            .expect("Failed to copy texture");

        // Make every unit think() of what to do,
        for u in &mut unit_list {
            u.think(terrain.clone(), delta_time);

            u.draw_at(&mut canvas, zoom)
                .expect("Cannot draw Unit for some reason :^)");
        }
        canvas.set_scale(zoom, zoom)?;
        canvas.present();
    }
}
