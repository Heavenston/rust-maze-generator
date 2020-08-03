use bitfield::*;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Maze {
    width: usize,
    height: usize,

    cursor: Position,
    tail: Vec<Position>,
    cells: Vec<MazeCell>,
    rng: SmallRng,
}
#[wasm_bindgen]
impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let mut default_cell = MazeCell::new();
        default_cell.set_bottom(true);
        default_cell.set_right(true);
        let mut this = Self {
            width,
            height,

            cursor: Position::new(0, 0),
            tail: vec![Position::new(0, 0)],
            cells: vec![default_cell; width * height],
            rng: SmallRng::from_entropy(),
        };
        this.cells[0].set_visited(true);
        this
    }
    pub fn from_seed(width: usize, height: usize, seed: u64) -> Self {
        let mut default_cell = MazeCell::new();
        default_cell.set_bottom(true);
        default_cell.set_right(true);
        let mut this = Self {
            width,
            height,

            cursor: Position::new(0, 0),
            tail: vec![Position::new(0, 0)],
            cells: vec![default_cell; width * height],
            rng: SmallRng::seed_from_u64(seed),
        };
        this.cells[0].set_visited(true);
        this
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells_ptr(&self) -> *const MazeCell {
        self.cells.as_ptr()
    }

    pub fn get_cell_offset(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
    pub fn get_cell(&self, x: usize, y: usize) -> MazeCell {
        self.cells[self.get_cell_offset(x, y)]
    }
    fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut MazeCell {
        let offset = self.get_cell_offset(x, y);
        &mut self.cells[offset]
    }

    pub fn gen_step(&mut self) -> bool {
        let mut next_pos = [
            Direction::Left,
            Direction::Right,
            Direction::Top,
            Direction::Bottom,
        ];
        next_pos.shuffle(&mut self.rng);
        let mut dest: Option<Direction> = None;

        for dir in next_pos.iter() {
            match dir {
                Direction::Bottom => {
                    if self.cursor.y >= self.height-1 {
                        continue;
                    };
                    let pos = Position::new(self.cursor.x, self.cursor.y + 1);
                    if self.get_cell(pos.x, pos.y).visited() {
                        continue;
                    }
                    dest = Some(Direction::Bottom);
                    break;
                }
                Direction::Left => {
                    if self.cursor.x == 0 {
                        continue;
                    };
                    let pos = Position::new(self.cursor.x - 1, self.cursor.y);
                    if self.get_cell(pos.x, pos.y).visited() {
                        continue;
                    }
                    dest = Some(Direction::Left);
                    break;
                }
                Direction::Right => {
                    if self.cursor.x >= self.width-1 {
                        continue;
                    };
                    let pos = Position::new(self.cursor.x + 1, self.cursor.y);
                    if self.get_cell(pos.x, pos.y).visited() {
                        continue;
                    }
                    dest = Some(Direction::Right);
                    break;
                }
                Direction::Top => {
                    if self.cursor.y == 0 {
                        continue;
                    };
                    let pos = Position::new(self.cursor.x, self.cursor.y - 1);
                    if self.get_cell(pos.x, pos.y).visited() {
                        continue;
                    }
                    dest = Some(Direction::Top);
                    break;
                }
            }
        }

        match dest {
            Some(dir) => {
                match dir {
                    Direction::Bottom => {
                        self.get_cell_mut(self.cursor.x, self.cursor.y).set_bottom(false);
                        dir.apply(&mut self.cursor);
                    }
                    Direction::Right => {
                        self.get_cell_mut(self.cursor.x, self.cursor.y).set_right(false);
                        dir.apply(&mut self.cursor);
                    }
                    Direction::Top => {
                        dir.apply(&mut self.cursor);
                        self.get_cell_mut(self.cursor.x, self.cursor.y).set_bottom(false);
                    }
                    Direction::Left => {
                        dir.apply(&mut self.cursor);
                        self.get_cell_mut(self.cursor.x, self.cursor.y).set_right(false);
                    }
                }
                self.get_cell_mut(self.cursor.x, self.cursor.y).set_visited(true);
                self.tail.push(self.cursor.clone());
            }
            None => match self.tail.pop() {
                None => return true,
                Some(pos) => self.cursor = pos,
            },
        }

        false
    }
    pub fn generate(&mut self, limit: Option<usize>) -> bool {
        for _ in 0..limit.unwrap_or(usize::MAX) {
            if self.gen_step() {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Top,
    Right,
    Left,
    Bottom,
}
impl Direction {
    pub fn apply(&self, pos: &mut Position) {
        match self {
            Direction::Top => {
                pos.y -= 1;
            }
            Direction::Right => {
                pos.x += 1;
            }
            Direction::Left => {
                pos.x -= 1;
            }
            Direction::Bottom => {
                pos.y += 1;
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

bitfield! {
    #[wasm_bindgen]
    #[derive(Clone, Copy, Debug)]
    pub struct MazeCell(u8);
    bool;
    // The fields default to u16
    pub visited, set_visited: 0;
    pub right, set_right: 1;
    pub bottom, set_bottom: 2;
}
impl MazeCell {
    pub fn new() -> Self {
        MazeCell(0)
    }
}
