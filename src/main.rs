extern crate tetrust;
extern crate ncurses;
use tetrust::field::Field;
use tetrust::mino::Mino;
use tetrust::command::{Command, Action};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use ncurses as nc;

fn main() {
    nc::initscr();
    let mut field = Field::new_field();
    let mut mino = Mino::new_mino(5, 0);
    let command = Arc::new(Mutex::new(Command::init()));
    input_thread(&command);
    loop {
        nc::refresh();
        thread::sleep(time::Duration::from_millis(200));

        println!("point: {}\x1b[25D", field.get_point());
        println!("left: f, right: j, rotate: space, q: break\x1b[25D");

        let mut command = command.lock().unwrap();
        let tmp = match command.get_comand() {
            Action::Left => mino.left(),
            Action::Right => mino.right(),
            Action::Rotation => mino.rotation(),
            Action::Fall => mino.fall(),
            Action::Quit => {
                command.game_finish();
                nc::endwin();
                break;
            },
            _ => mino.under(),
        };

        let next_position = tmp.move_position();
        if !field.are_block(&next_position) {
            field.pre_fix(&next_position);
            mino = tmp;
        }else if command.get_comand() != Action::None {
            command.none();
        }else{
            let current_pos = mino.move_position();
            field.fix(&current_pos);
            mino = Mino::new_mino(5, 0);
            let next_pos = mino.move_position();
            if field.are_block(&next_pos){
                command.game_finish();
                nc::endwin();
                println!("game over");
                break;
            }
        }

        if command.get_comand() != Action::None {
            command.none();
        }
        field.write();
        field.clear();
        field.line_clear();

        if field.game_finish(){
            command.game_finish();
            nc::endwin();
            println!("success!!!!!!!!!!");
            break;
        }
    }
}

fn input_thread(command: &Arc<Mutex<Command>>) {
        let command = command.clone();
        thread::spawn(move || {
            loop {
                let ch = nc::getch() as u8 as char;
                let mut command = command.lock().unwrap();
                if command.get_comand() == Action::Finished {
                    break;
                }
                match ch {
                    'f'  => command.left(),
                    'j'  => command.right(),
                    ' '  => command.rotation(),
                    'g'  => command.fall(),
                    'q'  => command.quit(),
                    _  => command.none()
                }
            }
        });
}
