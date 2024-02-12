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
    FruitCollison,
    SnakeCollison,
    NoneCollison,
}

pub struct Snake {
    pub coords: Vec<[i32; 2]>,
    pub score: i32,
    pub direction: Direction,
}

impl Snake {
    pub fn new(starting_coords: Vec<[i32; 2]>, starting_direction: Direction) -> Snake {
        Snake {
            coords: starting_coords,
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

        // Check if snake moves off screen
        if new_head[1] < 0 {
            new_head[1] = screen_size[1];
        } else if new_head[1] > screen_size[1] {
            new_head[1] = 0;
        } else if new_head[0] < 0 {
            new_head[0] = screen_size[0];
        } else if new_head[0] > screen_size[0] {
            new_head[0] = 0;
        }

        // Removes last element
        let _ = self.coords.pop();
        self.coords.insert(0, new_head);

        if self.test_fruit_collion(fruit) {
            Collision::FruitCollison
        } else if self.test_snake_collison() {
            Collision::SnakeCollison
        } else {
            Collision::NoneCollison
        }
    }

    fn test_fruit_collion(&mut self, fruit: Fruit) -> bool {
        // Check if `head` of snake has same coords as fruit
        if self.coords[0] == fruit.coords {
            return true;
        } else {
            return false;
        }
    }

    fn test_snake_collison(&mut self) -> bool {
        for coord in &self.coords[1..] {
            if &self.coords[0] == coord {
                return true;
            }
        }
        return false;
    }
}