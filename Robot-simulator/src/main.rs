#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    fn new(x: i32, y:i32, direction: Direction) -> Self {
        Robot {x,y,direction}

    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();

    }

    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    fn advance(&mut self) {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }

    fn execute_instructions(&mut self, instructions: &str) {
        for instruction in instructions.chars() {
            match instruction {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _=>(),
            }
        }

    }
}

fn main() {

}
