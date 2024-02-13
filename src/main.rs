pub mod snake;
pub mod fruit;
use snake::{Snake, Direction, Collision};
use fruit::Fruit;
use sdl2;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect};
use std::time::Duration;

const SCREEN_SIZE: [i32; 2] = [1280, 720];

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Snake Game", SCREEN_SIZE[0] as u32, SCREEN_SIZE[1] as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut snake = Snake::new(vec![[50, 0], [25, 0]], Direction::Down);
    let mut fruit = Fruit {coords: [100, 100]};

    'running: loop {
        for event in event_pump.poll_iter() {

            // Testing for keypresses
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    snake.change_direction(Direction::Up);
                }
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    snake.change_direction(Direction::Down);
                }
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    snake.change_direction(Direction::Left);
                }
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    snake.change_direction(Direction::Right);
                }
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    snake.change_direction(Direction::Up);
                }
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    snake.change_direction(Direction::Down);
                }
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    snake.change_direction(Direction::Left);
                }
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    snake.change_direction(Direction::Right);
                }
                _ => {}
            }
        }

        let collision = snake.move_snake(fruit, SCREEN_SIZE);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = rect::Rect::new(fruit.coords[0], fruit.coords[1], 20, 20);
        canvas.fill_rect(rect).unwrap();

        let mut i = 0;
        for coord in &snake.coords {
            if i == 0 {
                canvas.set_draw_color(Color::RGB(0, 255, 0));
                i += 1;
            } else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            }

            let rect = rect::Rect::new(coord[0], coord[1], 20, 20);
            canvas.fill_rect(rect).unwrap();
        }

        

        if collision == Collision::FruitCollision {
            fruit.move_to_random_location(SCREEN_SIZE);
            println!("Fruit!");
            snake.grow();
            snake.increase_score();
            print!("Score: {}", snake.score);
        } else if collision == Collision::SnakeCollision {
            println!("Snake collision!");
            break 'running;
        }         

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}