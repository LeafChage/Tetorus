/*
 * 入力のために作った構造体のファイル
 * */

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Right,    // 右に動く
    Left,     //左に動く
    Fall,     // 下に落ちる
    Rotation, // 回転する
    Quit,     //ゲームを終了する
    Finished,   //ゲームが終了した
    None
}

pub struct Command {
    command: Action
}

impl Command {
    pub fn get_comand(&self) -> Action {
        self.command.clone()
    }
    pub fn init() -> Self {
        Self{ command: Action::None }
    }
    pub fn right(&mut self){
        self.command = Action::Right;
    }
    pub fn left(&mut self){
        self.command = Action::Left;
    }
    pub fn fall(&mut self){
        self.command = Action::Fall;
    }
    pub fn rotation(&mut self){
        self.command = Action::Rotation;
    }
    pub fn none(&mut self){
        self.command = Action::None;
    }
    pub fn quit(&mut self) {
        self.command = Action::Quit;
    }
    pub fn game_finish(&mut self) {
        self.command = Action::Finished;
    }
}

