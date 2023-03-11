use random::Source;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("8=D", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(20, 24, 22));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // let mut source = random::default(42);
    let mut pos1 = Position {
        x: 10,
        y: 10,
    };

    let speed = Speed {
        value: 4,
    };

    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        //println!("Random number: {}", source.read::<i8>());
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(20, 24, 22));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    pos1.y -= 1 * speed.value;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    pos1.x -= 1 * speed.value;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    pos1.y += 1 * speed.value;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    pos1.x += 1 * speed.value;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let p1 = Point::new(0, 0);
        let p2 = Point::new(pos1.x, pos1.y);
        let rect1 = Rect::new(20, 20, 200, 400);
        _ = canvas.draw_line(p1, p2);
        _ = canvas.draw_rect(rect1);

        

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

struct Position {
    x: i32,
    y: i32,
}

struct Speed {
    value: i32,
}
