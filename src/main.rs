//mod shaders;
//use shaders::create_shader;

use std::thread;
use std::time::{Duration, Instant};

mod units;
use terrain::Terrain;
use units::{ActionType, Actions, Coords, JobType, RaceType, Unit};

mod window;

mod terrain;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

enum Directions {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn main() {
    let mut unit_list: Vec<Unit> = Vec::new();

    for i in 0..100 {
        let mut unit = Unit::new(
            if i % 3 == 0 {
                RaceType::HUMAN
            } else if i % 3 == 1 {
                RaceType::ANT
            } else {
                RaceType::ALIEN
            },
            JobType::MINER,
            Coords { x: 400, y: 300 },
        );
        // add move actions for testing
        for _ in 0..10 {
            unit.action_queue.push(ActionType::WANDER);
        }
        unit_list.push(unit);
    }

    let (sdl_context, window) = window::init_sdl2_window();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut last_time = Instant::now();

    let texture_creator = canvas.texture_creator();
    let mut terrain_texture = texture_creator
        .create_texture_target(None, 800, 600)
        .expect("Failed to create texture");

    let mut terrain = Terrain::new(800, 600);
    terrain.generate();

    canvas
        .with_texture_canvas(&mut terrain_texture, |texture_canvas| {
            terrain.draw(texture_canvas);
        })
        .expect("Failed to draw terrain on texture");

    'running: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_millis() as i32;
        last_time = current_time;

        // SDL2 events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas
            .copy(&terrain_texture, None, None)
            .expect("Failed to copy texture");

        // Make every unit think() of what to do,
        for u in &mut unit_list {
            u.think(delta_time);
            u.draw(&mut canvas)
                .expect("Cannot draw Unit for some reason :^)");
        }

        canvas.present();
    }
}
