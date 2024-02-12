pub mod snake;
pub mod fruit;
use snake::{Snake, Direction};
use fruit::Fruit;
use sdl2;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect};
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Snake Game", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut snake = Snake::new(vec![[50, 0], [25, 0]], Direction::Down);
    let fruit = Fruit {coords: [100, 100]};

    'running: loop {
        for event in event_pump.poll_iter() {

            // Testing for keypresses
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    snake.direction = Direction::Up;
                }
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    snake.direction = Direction::Down;
                }
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    snake.direction = Direction::Left;
                }
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    snake.direction = Direction::Right;
                }
                
                _ => {}
            }
        }

        let collision = snake.move_snake(fruit);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for coord in &snake.coords {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let rect = rect::Rect::new(coord[0], coord[1], 25, 25);
            canvas.fill_rect(rect).unwrap();
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
