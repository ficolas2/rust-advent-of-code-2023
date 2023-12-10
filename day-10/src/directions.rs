
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn get_vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

pub fn get_connected_directions(c: &char) -> Vec<Direction> {
    match c {
        '|' => vec![Direction::North, Direction::South],
        '─' => vec![Direction::East, Direction::West],
        '└' => vec![Direction::North, Direction::East],
        '┘' => vec![Direction::North, Direction::West],
        '┐' => vec![Direction::South, Direction::West],
        '┌' => vec![Direction::South, Direction::East],
        '.' => vec![],
        'S' => vec![Direction::North, Direction::East, Direction::South, Direction::West],
        _ => panic!("Invalid character"),
    }
}


pub const DIRECTIONS : [Direction; 4] = [ Direction::North, Direction::East, Direction::South, Direction::West ];

