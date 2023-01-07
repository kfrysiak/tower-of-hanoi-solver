use colored::{Colorize};
use stack::Stack;
use screen::Screen;
use std::io;
use std::io::prelude::*;
use itertools::Itertools;

mod stack;
mod screen;
mod solver;

pub const IS_INTERACTIVE: bool = true;
pub const IS_VERBOSE: bool = false;
pub const USE_TUI: bool = true;

fn main() {
    let mut red = Stack::new_of_size("Red", 5, colored::Color::Red);
    let mut blue = Stack::new("Blue", vec![], colored::Color::Blue);
    let mut yellow = Stack::new("Yellow", vec![], colored::Color::Yellow);
    let mut step: u32 = 0;

    display_stacks(&step, &vec![&red, &blue, &yellow]);
    solver::move_stack(&mut red, &mut blue, &mut yellow, &mut step, None).unwrap();
}


fn display_stacks(step: &u32, stacks: &Vec<&Stack>) {
    println!("===== Step {} =====", step);
    if USE_TUI {
        let max_height = stacks.iter().fold(0, |acc, &stack| acc + stack.rings.len());
        let max_width = (max_height) * 2 + 1;

        let mut screen = Screen::new(max_height+1, (max_width + 2) * stacks.len());
        let mut stack_index = 0;
        for &stack in stacks.iter().sorted_by(|&a, &b| a.name.cmp(&b.name)) {
            screen.print_stack(
                &stack,
                max_height,
                (max_width + 2) * stack_index,
                0,
            );
            stack_index += 1;
        }

        println!("{}", screen);
        return;
    }
    for &stack in stacks.iter().sorted_by(|&a, &b| a.name.cmp(&b.name)) {
        println!("{}    {}", stack.name.color(stack.color), stack);
    }
}


fn pause(comment: &str) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "{} - Press to continue", comment).unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
