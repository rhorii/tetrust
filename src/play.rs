use std::sync::{Arc, Mutex};
use std::{thread, time};
use getch_rs::{Getch, Key};
use crate::game::*;
use crate::ai::eval;

pub fn normal() {
    let game = Arc::new(Mutex::new(Game::new()));

    println!("\x1b[2J\x1b[H\x1b[?25l"); // clear display
    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || {
            loop {
                let sleep_msec = match 1000u64.saturating_sub((game.lock().unwrap().line as u64 / 10) * 100) {
                    0 => 100,
                    msec => msec,
                };
                thread::sleep(time::Duration::from_millis(sleep_msec));

                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, &game.block) {
                    game.pos = new_pos;
                } else {
                    if landing(&mut game).is_err() {
                        gameover(&game);
                    }
                }
                draw(&game);
            }
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Up) => {
                let mut game = game.lock().unwrap();
                hard_drop(&mut game);
                if landing(&mut game).is_err() {
                    gameover(&game);
                }
                draw(&game);
            }
            Ok(Key::Char('z')) => {
                let mut game = game.lock().unwrap();
                rotate_left(&mut game);
                draw(&game);
            }
            Ok(Key::Char('x')) => {
                let mut game = game.lock().unwrap();
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Char(' ')) => {
                let mut game = game.lock().unwrap();
                hold(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                break;
            },
            _ => (),
        }
    }

    quit();
}

pub fn auto() {
    let _ = thread::spawn(|| {
        let mut game = Game::new();
        println!("\x1b[2J\x1b[H\x1b[?25l"); // clear display
        draw(&game);

        loop {
            let elite = eval(&game, &[100, 1, 10, 100]);
            game = elite;

            if landing(&mut game).is_err() {
                gameover(&game);
                break;
            }

            draw(&game);
        }
    });

    let g = Getch::new();
    loop {
        if let Ok(Key::Char('q')) = g.getch() {
            break;
        }
    }

    quit();
}
