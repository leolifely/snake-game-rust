use crate::fruit::Fruit;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Snake {
    coords: Vec<[i32; 2]>,
    score: i32,
    direction: Direction,
}

impl Snake {
    pub fn new(starting_coords: Vec<[i32; 2]>, starting_direction: Direction) -> Snake {
        Snake {
            coords: starting_coords,
            score: 0,
            direction: starting_direction,
        }
    }

    pub fn move_snake(&mut self) {
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
        let new_head = [self.coords[0][0] + speed[0], self.coords[0][1] + speed[1]];

        // Removes last element
        let _ = self.coords.pop();
        self.coords.insert(0, new_head)
    }

    pub fn test_fruit_collision(&mut self, fruit: Fruit) -> bool {
        // Check if `head` of snake has same coords as fruit
        if self.coords[0] == fruit.coords {
            true
        } else {
            false
        }
    }
}