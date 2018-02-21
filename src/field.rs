use mino;

#[derive(PartialEq)]
enum Status {
    None, // 何もない
        Wall, // 壁
        Active, // まだ動いている途中
        Fix // 場所が決まってもう動かない
}

type Line = [Status; WIDTH];
type FieldBlocks = Vec<Line>;
pub struct Field {
    point: usize,
    field: FieldBlocks
}

const HEIGHT: usize = 20;
const WIDTH: usize = 11;
const FINISH_POINT: usize = 40;
const DELETE_LINE: Line = [ Status::Wall, Status::Fix, Status::Fix, Status::Fix, Status::Fix, Status::Fix, Status::Fix, Status::Fix, Status::Fix, Status::Fix, Status::Wall];
const NEW_LINE: Line = [ Status::Wall, Status::None, Status::None, Status::None, Status::None, Status::None, Status::None, Status::None, Status::None, Status::None, Status::Wall];
const FLOOR_LINE: Line = [ Status::Wall, Status::Wall, Status::Wall, Status::Wall, Status::Wall, Status::Wall, Status::Wall, Status::Wall, Status::Wall, Status::Wall, Status::Wall];

impl Field {
    pub fn new_field() -> Self {
        Self {
            point: 0,
            field: Self::init_field()
        }
    }
    pub fn get_point(&self) -> usize {
        self.point
    }
    pub fn clear(&mut self) -> &Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.field[y][x] == Status::Active {
                    self.field[y][x] = Status::None;
                }
            }
        }
        self
    }

    pub fn write(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.field[y][x] == Status::Wall {
                    print!("# ");
                }else if self.field[y][x] == Status::None {
                    print!("_ ");
                }else{
                    print!("o ");
                }
            }
            print!("\n\x1b[50D");
        }
    }

    pub fn line_clear(&mut self) -> &Self {
        for y in 0..(self.field.len()) {
            if self.field[y] == DELETE_LINE {
                self.point += 1;
                self.field.remove(y);
                self.field.insert(0, NEW_LINE);
            }
        }
        self
    }

    pub fn game_finish(&self) -> bool {
        self.point >= FINISH_POINT
    }

    pub fn are_block(&self, pos: &mino::NextPosition) -> bool {
        for p in pos.iter() {
            if self.is_block(p[0], p[1]) {
                return true
            }
        }
        false
    }

    fn is_block(&self, x: usize, y: usize) -> bool {
        self.field[y][x] == Status::Wall || self.field[y][x] == Status::Fix
    }

    pub fn fix(&mut self, pos: &mino::NextPosition) -> &Self {
        for p in pos {
            self.field[p[1]][p[0]] = Status::Fix;
        }
        self
    }

    pub fn pre_fix(&mut self, pos: &mino::NextPosition) -> &Self {
        for p in pos {
            let x = p[0] as usize;
            let y = p[1] as usize;
            self.field[y][x] = Status::Active;
        }
        self
    }

    fn init_field() -> FieldBlocks {
        let mut blocks: FieldBlocks = Vec::with_capacity(HEIGHT);
        for _ in 0..(HEIGHT-1) {
            blocks.push(NEW_LINE)
        }
        blocks.push(FLOOR_LINE);
        blocks
    }

}

