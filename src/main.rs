extern crate sdl2;
pub mod snake;
pub mod fruit;
use snake::{Snake, Direction, Collision};
use fruit::Fruit;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect, ttf};
use std::time::Duration;
use std::path::Path;
use rand::Rng;

const SCREEN_SIZE: [i32; 2] = [1300, 725];

fn main() {

    // Initializing SDL2
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

    // Initializing event pump, snake and fruit
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut snake = Snake::new(vec![[50, 25], [75, 25]], vec![Color::RGB(0, 255, 0), Color::RGB(255, 255, 255)], Direction::Down);
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
                Event::KeyDown {keycode: Some(Keycode::Backspace), ..} => {
                    snake.grow(get_random_colour());
                }
                _ => {}
            }
        }

        let collision = snake.move_snake(fruit, SCREEN_SIZE);

        // Setting whole screen to black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Drawing fruit
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let fruit_rect = rect::Rect::new(fruit.coords[0], fruit.coords[1], 20, 20);
        canvas.fill_rect(fruit_rect).unwrap();

        // Drawing snake
        for (i, coord) in snake.get_coords().iter().enumerate() {
            canvas.set_draw_color(snake.get_colours()[i]);
            let rect = rect::Rect::new(coord[0], coord[1], 20, 20);
            canvas.fill_rect(rect).unwrap();
        }

        // Testing for collisions
        if collision == Collision::FruitCollision {
            fruit.move_to_random_location(SCREEN_SIZE);
            snake.grow(get_random_colour());
            print!("Score: {}", snake.get_score());
        } else if collision == Collision::SnakeCollision {
            println!("Snake collision!");
            break 'running;
        }

        // Drawing score and updating screen
        draw_score(snake.get_score(), &mut canvas);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}

fn draw_score(score: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // Draws `score` to canvas

    let font_path: &Path = Path::new("fonts/Roboto-Regular.ttf");
    let texture_creator = canvas.texture_creator();
    let ttf_context = ttf::init().unwrap();
    let font = ttf_context.load_font(font_path, 128).unwrap();
    let surface = font.render(format!("{}", score).as_str())
        .blended(Color::RGBA(255, 255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    canvas.copy(&texture, None, rect::Rect::new(0, 0, 50 * format!("{}", score).as_str().len() as u32, 100)).unwrap();
}

fn get_random_colour() -> Color {
    // Returns a random colour with low levels of green
    let mut rng = rand::thread_rng();
    Color::RGB(rng.gen_range(10..255), rng.gen_range(10..100), rng.gen_range(10..255))
}