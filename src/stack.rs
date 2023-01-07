use colored::{ Colorize};
use itertools::Itertools;
use std::fmt;

pub struct Stack {
    pub rings: Vec<usize>,
    pub color: colored::Color,
    pub name: String,
}

impl Stack {
    pub fn new(name: &str, rings: Vec<usize>, color: colored::Color) -> Self {
        return Self {
            rings,
            color,
            name: String::from(name),
        };
    }
    pub fn new_of_size(name: &str, size: usize, color: colored::Color) -> Self {
        let mut rings = vec![];
        for ring in (1..=size).rev() {
            rings.push(ring);
        }
        return Self {
            rings,
            color,
            name: String::from(name),
        };
    }


pub fn move_ring(&mut self, target: &mut Stack) -> Result<(), &'static str> {
    if crate::IS_VERBOSE {
        println!(
            "== Move {} [{}] -> {} {}",
            self.name.color(self.color),
            self,
            target.name.color(target.color),
            target
        );
    } else {
        println!(
            "== Move {} -> {}",
            self.name.color(self.color),
            target.name.color(target.color)
        );
    }
    if crate::IS_INTERACTIVE {
        crate::pause("");
        print!("{}[2J", 27 as char);
    }
    if self.rings.len() == 0 {
        return Err("Nothing to move");
    }
    if !target.rings.is_empty() && self.rings.last() > target.rings.last() {
        return Err("Cannot put a larger ring on a smaller one");
    }
    target.rings.push(self.rings.pop().unwrap());
    return Ok(());
}
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.rings
                .iter()
                .map(|ring| { ring.to_string().color(crate::screen::get_ring_color(ring)) })
                .join(", ")
        )?;
        Ok(())
    }
}

