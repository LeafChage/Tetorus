use rand::{StdRng, Rng};

type MinoBlock = [[usize; 4]; 4];
pub type NextPosition = [[usize; 2]; 4];
pub struct Mino {
    x: usize,
    y: usize,
    current: MinoBlock
}

const BLOCKS: [MinoBlock; 7]  = [
        [ [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        [ [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        [ [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        [ [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        [ [1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        [ [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        [ [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        ];

impl Mino {
    pub fn new_mino(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
            current: Self::random_choice()
        }
    }
    pub  fn copy_mino(x: usize, y: usize, current: MinoBlock) -> Self {
        Self {
            x: x,
            y: y,
            current: current
        }
    }

    pub fn move_position(&self) -> NextPosition {
        let mut positions: NextPosition = [[0; 2]; 4];
        let mut index = 0;
        for y in 0..4 {
            for x in 0..4 {
                if self.current[y][x] == 1 {
                    positions[index][0] = self.x + x;
                    positions[index][1] = self.y + y;
                    index += 1;
                }
            }
        }
        positions
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            current: self.current.clone()
        }
    }
    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            current: self.current.clone()
        }
    }
    pub fn under(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            current: self.current.clone()
        }
    }

    pub fn fall(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            current: self.current.clone()
        }
    }

    pub fn rotation(&self) -> Self {
        let mut block: MinoBlock = [[0;4];4];
        for y in 0..4 {
            for x in 0..4 {
                block[x][y] = self.current[y][x];
            }
        }
        for y in 0..4 {
            block[y].reverse();
        }
        Self {
            x: self.x,
            y: self.y,
            current: block
        }
    }

    pub fn random_choice() -> MinoBlock {
        let mut rng = StdRng::new().expect("StdRng::new() error.");
        BLOCKS[rng.gen_range(0, BLOCKS.len())]
    }
}


