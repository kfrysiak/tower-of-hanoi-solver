use crate::stack::Stack;
use colored::Color;
use itertools::Itertools;
use std::fmt;

type RingPieces = (&'static str, &'static str, &'static str);
type StackPieces = (&'static str, &'static str, &'static str);

//   │      ║
// ──┴──  ══╩══

static GREEN_STACK_PIECES:StackPieces = (
    "\x1b[32;1m║\x1b[0m",
    "\x1b[32;1m╩\x1b[0m",
    "\x1b[32;1m═\x1b[0m",
);
static GREEN_RING: RingPieces = (
    "\x1b[32;1m╭\x1b[0m",
    "\x1b[32;1m─\x1b[0m",
    "\x1b[32;1m╮\x1b[0m",
);
static YELLOW_RING: RingPieces = (
    "\x1b[33m╭\x1b[0m",
    "\x1b[33m─\x1b[0m",
    "\x1b[33m╮\x1b[0m",
);
static YELLOW_STACK_PIECES: StackPieces = (
    "\x1b[33m║\x1b[0m",
    "\x1b[33m╩\x1b[0m",
    "\x1b[33m═\x1b[0m",
);
static BLUE_STACK_PIECES: StackPieces = (
    "\x1b[34m║\x1b[0m",
    "\x1b[34m╩\x1b[0m",
    "\x1b[34m═\x1b[0m",
);
static BLUE_RING: RingPieces = (
    "\x1b[34m╭\x1b[0m",
    "\x1b[34m─\x1b[0m",
    "\x1b[34m╮\x1b[0m",
);
static RED_RING: RingPieces = (
    "\x1b[31m╭\x1b[0m",
    "\x1b[31m─\x1b[0m",
    "\x1b[31m╮\x1b[0m",
);
static RED_STACK_PIECES: StackPieces = (
    "\x1b[31m║\x1b[0m",
    "\x1b[31m╩\x1b[0m",
    "\x1b[31m═\x1b[0m",
);
static WHITE_RING: RingPieces = ("╭", "─", "╮");
static WHITE_STACK_PIECES: StackPieces = ("║", "╩", "═");
static EMPTY: &str = " ";

pub struct Screen<'a> {
    vec: Vec<&'a str>,
    col: usize,
    row: usize,
}

impl<'a> fmt::Display for Screen<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_index in 0..self.row {
            writeln!(f, "{}", self.row(row_index).into_iter().join(""))?;
        }

        Ok(())
    }
}

impl<'a> Screen<'a> {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            vec: vec![EMPTY; row * col],
            row,
            col,
        }
    }

    pub fn row(&self, row: usize) -> &[&str] {
        let i = self.col * row;
        &self.vec[i..(i + self.col)]
    }

    pub fn index(&self, row: usize, col: usize) -> &&str {
        let i = self.col * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&'a mut self, row: usize, col: usize) -> &'a mut &str {
        let i = self.col * row;
        &mut self.vec[i + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: &'a str){
        let i = self.col * row;
        self.vec[i + col] = value;
    }

    pub fn print_ring(&mut self, ring: usize, offset_x: usize, offset_y: usize) {
        let ring_width = (ring) * 2 + 1;
        let half_width = (ring_width - 1) / 2;

        let pieces = get_ring_pieces(&ring);
        self.set(offset_y,offset_x, pieces.1);
        for i in 1..=half_width {
            self.set(offset_y, offset_x - i, 
                if i == half_width { pieces.0 } else { pieces.1 });
        }
        for i in 1..=half_width {
            self.set(offset_y, offset_x + i,
                if i == half_width { pieces.2 } else { pieces.1 });
        }
    }

    pub fn print_stack(
        &mut self,
        stack: &Stack,
        largest_ring: usize,
        offset_x: usize,
        offset_y: usize,
    ) {
        let largest_ring_width = (largest_ring) * 2 + 1;
        let half_width = (largest_ring_width - 1) / 2;

        let mut ring_index = 0;
        for ring in stack.rings.iter() {
            self.print_ring(
                (*ring).try_into().unwrap(),
                offset_x + half_width,
                offset_y + largest_ring - 1 - ring_index,
            );
            ring_index += 1;
        }
        let stack_pieces = get_stack_pieces(&stack.color);
        for stack_index in stack.rings.len()..largest_ring {
            if stack_index == 0 {
                self.set(offset_y + largest_ring - stack_index-1, offset_x + half_width, stack_pieces.0);
            } else {
                self.set(offset_y + largest_ring - stack_index-1, offset_x + half_width, stack_pieces.0);
            }
        }
        let stack_half_width = 2;
        for i in 1..=stack_half_width {
            self.set(offset_y + largest_ring, offset_x + half_width - i, stack_pieces.2);
        }
        for i in 1..=stack_half_width {
            self.set(offset_y + largest_ring, offset_x + half_width + i, stack_pieces.2);
        }

        if stack.rings.len() == 0 {
            self.set(offset_y + largest_ring, offset_x + half_width, stack_pieces.1);
        } else {
            self.set(offset_y + largest_ring, offset_x + half_width, stack_pieces.2);
        }
    }
}

pub fn get_ring_color(ring_size: &usize) -> Color {
    let modulo = ring_size % 4;
    return match modulo {
        0 => Color::Red,
        1 => Color::BrightGreen,
        2 => Color::Yellow,
        3 => Color::Blue,
        _ => Color::White,
    };
}

pub fn get_ring_pieces(ring_size: &usize) -> &RingPieces {
    match get_ring_color(ring_size) {
        Color::Red => &RED_RING,
        Color::Blue => &BLUE_RING,
        Color::BrightGreen => &GREEN_RING,
        Color::Yellow => &YELLOW_RING,
        _ => &WHITE_RING,
    }
}

pub fn get_stack_pieces(color: &Color) -> &StackPieces {
    match color {
        Color::Red => &RED_STACK_PIECES,
        Color::Blue => &BLUE_STACK_PIECES,
        Color::Yellow => &YELLOW_STACK_PIECES,
        Color::Green => &GREEN_STACK_PIECES,
        _ => &WHITE_STACK_PIECES,
    }
}
