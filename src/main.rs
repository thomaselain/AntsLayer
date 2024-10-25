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

    for i in 0..20 {
        let mut unit = Unit::new(
            if i % 2 == 0 {
                RaceType::HUMAN
            } else {
                RaceType::ANT
            },
            JobType::MINER,
            Coords {
                x: 400 + i * 20,
                y: 300,
            },
        );
        // add move actions for testing
        for _ in 0..2 {
            unit.action_queue.push(ActionType::MOVE);
        }
        unit_list.push(unit);
    }

    let (sdl_context, window) = window::init_sdl2_window();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut last_time = Instant::now();

    let mut terrain = Terrain::new(800, 600);
    terrain.generate();

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

        // draw terrain
        terrain.draw(&mut canvas);

        // Make every unit think() of what to do,
        for u in &mut unit_list {
            u.think(delta_time);
            u.draw(&mut canvas)
                .expect("Cannot draw Unit for some reason :^)");
        }

        canvas.present();
    }
}
