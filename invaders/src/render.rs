use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::frame::Frame;

fn queue_write_to(stdout: &mut Stdout, x: usize, y: usize, s: &str) {
    stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
    print!("{}", s);
}

pub fn clear_and_render(stdout: &mut Stdout, curr_frame: &Frame) {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, &s) in col.iter().enumerate() {
            queue_write_to(stdout, x, y, s);
        }
    }
    stdout.flush().unwrap();
}

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame) {
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, &s) in col.iter().enumerate() {
            if s != last_frame[x][y] {
                queue_write_to(stdout, x, y, s);
            }
        }
    }
    stdout.flush().unwrap();
}
