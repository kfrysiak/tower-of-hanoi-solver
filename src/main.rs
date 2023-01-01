use colored::{Colorize, Color};
use itertools::{sorted, Itertools};
use std::io;
use std::io::prelude::*;
use std::fmt;

static IS_INTERACTIVE: bool = false;
static IS_VERBOSE: bool = false;

struct Stack {
    rings: Vec<u32>,
    color: colored::Color,
    name: String,
}

impl Stack {
    fn new(name: &str, rings: Vec<u32>, color: colored::Color) -> Self {
        return Self {
            rings,
            color,
            name: String::from(name),
        };
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Stack {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
            write!(f, "{}", self.rings.iter().map(|ring|{
                ring.to_string().color(get_ring_color(ring))
            }).join(", "))?;
            // write!(f, "{}", ))?;
        Ok(())
    }
}
fn main() {
    // let mut red = Stack::new("Red", vec![4, 3, 2, 1], colored::Color::Red);
    let mut red = Stack::new("Red", vec![6, 5, 4, 3, 2, 1], colored::Color::Red);
    let mut blue = Stack::new("Blue", vec![], colored::Color::Blue);
    let mut yellow = Stack::new("Yellow", vec![], colored::Color::Yellow);
    let mut step: u32 = 0;

    display_stacks(&step, &vec![&red, &blue, &yellow]);
    move_stack(&mut red, &mut blue, &mut yellow, &mut step, None).unwrap();
    display_stacks(&step, &vec![&red, &blue, &yellow]);
}

fn move_stack(
    source: &mut Stack,
    target: &mut Stack,
    helper_stack: &mut Stack,
    step: &mut u32,
    max_depth: Option<&usize>,
) -> Result<(), &'static str> {
    let depth = if max_depth.is_none() {
        source.rings.len()
    } else {
        max_depth.unwrap().clone()
    };
    if IS_VERBOSE {
        println!("Move stack {}:{} -> {}", source.name, depth, target.name);
    }
    if source.rings.len() == 1 || depth == 1 {
        *step += 1;
        let move_result = move_ring(source, target);
        display_stacks(&step, &vec![&source, &target, &helper_stack]);
        return move_result;
    }
    let next_depth = if max_depth.is_none() {
        source.rings.len() - 1
    } else {
        max_depth.unwrap().clone() - 1
    };
    let result = move_stack(source, helper_stack, target, step, Some(&next_depth));

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    *step += 1;
    if IS_VERBOSE {
        println!("Move uncovered {} -> {}", source.name, target.name);
    }
    move_ring(source, target)?;
    display_stacks(&step, &vec![&source, &target, &helper_stack]);
    if IS_VERBOSE {
        println!(
            "Stack to uncovered {}:{}:{} -> {}",
            helper_stack.name, &depth, &next_depth, target.name
        );
    }
    return move_stack(helper_stack, target, source, step, Some(&next_depth));
}

fn display_stacks(step: &u32, stacks: &Vec<&Stack>) {
    println!("===== Step {} =====", step);

    for stack in stacks.iter().sorted_by(|&a, &b| a.name.cmp(&b.name)) {
        println!("{}    {}", stack.name.color(stack.color), stack);
    }
}

fn get_ring_color(ring_size: &u32) -> Color{
    let modulo = ring_size % 4;
    return match modulo {
        0 => Color::Red,
        1 => Color::BrightGreen,
        2 => Color::Yellow,
        3 => Color::Blue,
        _ => Color::White
    };
}

fn move_ring(source: &mut Stack, target: &mut Stack) -> Result<(), &'static str> {
    if IS_VERBOSE {
        println!(
            "== Move {} [{}] -> {} {}",
            source.name.color(source.color), source, target.name.color(target.color), target
        );
    } else {
        println!(
            "== Move {} -> {}",
            source.name.color(source.color), target.name.color(target.color)
        );
    }
    if IS_INTERACTIVE {
        pause("");
    }
    if source.rings.len() == 0 {
        return Err("Nothing to move");
    }
    if !target.rings.is_empty() && source.rings.last() > target.rings.last() {
        return Err("Cannot put a larger ring on a smaller one");
    }
    target.rings.push(source.rings.pop().unwrap());
    return Ok(());
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
