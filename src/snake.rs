use crate::fruit::Fruit;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
pub enum Collision {
    FruitCollision,
    SnakeCollision,
    NoneCollision,
}

pub struct Snake {
    coords: Vec<[i32; 2]>,
    colours: Vec<sdl2::pixels::Color>,
    score: i32,
    direction: Direction,
}

impl Snake {
    pub fn new(starting_coords: Vec<[i32; 2]>, starting_colours: Vec<sdl2::pixels::Color>, starting_direction: Direction) -> Snake {
        Snake {
            coords: starting_coords,
            colours: starting_colours,
            score: 0,
            direction: starting_direction,
        }
    }

    pub fn move_snake(&mut self, fruit: Fruit, screen_size: [i32; 2]) -> Collision {
        // Moves snake based on `self.direction`

        let speed: [i32; 2];
        if self.direction == Direction::Up {
            speed = [0, -25];
        } else if self.direction == Direction::Down {
            speed = [0, 25];
        } else if self.direction == Direction::Left {
            speed = [-25, 0];
        } else {
            speed = [25, 0];
        }

        // Adding speed to current `head`
        let mut new_head = [self.coords[0][0] + speed[0], self.coords[0][1] + speed[1]];

        // Check if snake moves off-screen
        if new_head[1] < 0 {
            new_head[1] = screen_size[1] - 25;
        } else if new_head[1] >= screen_size[1] {
            new_head[1] = 0;
        } else if new_head[0] < 0 {
            new_head[0] = screen_size[0] - 25;
        } else if new_head[0] >= screen_size[0] {
            new_head[0] = 0;
        }
        // Removes last element
        let _ = self.coords.pop();
        self.coords.insert(0, new_head);

        if self.test_fruit_collision(fruit) {
            Collision::FruitCollision
        } else if self.test_snake_collision() {
            Collision::SnakeCollision
        } else {
            Collision::NoneCollision
        }
    }

    pub fn grow(&mut self, colour: sdl2::pixels::Color) {
        self.score += 1;
        let len = self.coords.len();
        match self.direction {
            Direction::Down => {self.coords.push([self.coords[len-1][0], self.coords[len-1][1] - 25]);
            self.colours.push(colour);
            },
            Direction::Up => {self.coords.push([self.coords[len-1][0], self.coords[len-1][1] + 25]);
            self.colours.push(colour);
            },
            Direction::Left => {self.coords.push([self.coords[len-1][0] + 25, self.coords[len-1][1]]);
            self.colours.push(colour);
            },
            Direction::Right => {self.coords.push([self.coords[len-1][0] - 25, self.coords[len-1][1]]);
            self.colours.push(colour);
            },
        }
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
    }

    fn test_fruit_collision(&mut self, fruit: Fruit) -> bool {
        // Check if `head` of snake has same coords as fruit
        if self.coords[0] == fruit.coords {
            return true;
        } else {
            return false;
        }
    }

    fn test_snake_collision(&mut self) -> bool {
        for coord in &self.coords[1..] {
            if &self.coords[0] == coord {
                return true;
            }
        }
        return false;
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // Change direction of snake
        if new_direction == Direction::Up && self.direction != Direction::Down {
            self.direction = new_direction;
        } else if new_direction == Direction::Down && self.direction != Direction::Up {
            self.direction = new_direction;
        } else if new_direction == Direction::Left && self.direction != Direction::Right {
            self.direction = new_direction;
        } else if new_direction == Direction::Right && self.direction != Direction::Left {
            self.direction = new_direction;
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_coords(&self) -> &Vec<[i32; 2]> {
        &self.coords
    }

    pub fn get_colours(&self) -> &Vec<sdl2::pixels::Color> {
        &self.colours
    }
}